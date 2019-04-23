use std::ffi::{CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use derive_more::Display;
use errno::{errno, Errno};
use libc::{self, gid_t, passwd, uid_t, EPERM, ERANGE, _SC_GETPW_R_SIZE_MAX};

#[derive(Debug, Display, Clone, Copy)]
pub enum UserSwitchError {
    #[display(fmt = "User not found")]
    NotFound,
    #[display(fmt = "Insufficient privileges")]
    NotPermitted,
    #[display(fmt = "{} error: {}", "_0", "_1")]
    Error(&'static str, Errno),
}

#[derive(Debug, Display)]
#[display(
    fmt = "{}{}",
    "name.to_string_lossy()",
    "ids.map(|id| format!(\" ({})\", id)).unwrap_or_default()"
)]
pub struct User {
    pub name: OsString,
    ids: Result<UserIds, UserSwitchError>,
}

#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "uid={}, gid={}", uid, gid)]
struct UserIds {
    uid: uid_t,
    gid: gid_t,
}

impl UserIds {
    fn lookup(user: &OsString) -> Result<Self, UserSwitchError> {
        // Usernames containing NULL bytes cannot exist.
        let username = CString::new(user.as_bytes()).map_err(|_| UserSwitchError::NotFound)?;
        let mut pwd: passwd = unsafe { std::mem::zeroed() };
        let mut pwent: *mut passwd = std::ptr::null_mut();

        let mut size = unsafe { libc::sysconf(_SC_GETPW_R_SIZE_MAX) };
        if size < 1 || size > 1024 * 1024 {
            size = 1024;
        }
        let mut pwbuf = vec![0; size as usize];

        let mut e;

        loop {
            e = unsafe {
                libc::getpwnam_r(
                    username.as_ptr(),
                    &mut pwd,
                    pwbuf.as_mut_ptr(),
                    pwbuf.len(),
                    &mut pwent,
                )
            };

            if e == ERANGE && pwbuf.len() < 1024 * 1024 {
                pwbuf.resize(pwbuf.len() * 2, 0);
            } else {
                break;
            }
        }

        if e != 0 {
            Err(UserSwitchError::Error("getpwnam_r()", Errno(e as i32)))
        } else if pwent.is_null() {
            Err(UserSwitchError::NotFound)
        } else {
            Ok(UserIds {
                uid: pwd.pw_uid,
                gid: pwd.pw_gid,
            })
        }
    }
}

impl<'a, T: AsRef<OsStr>> From<T> for User {
    fn from(name: T) -> Self {
        let name = name.as_ref().to_owned();
        let ids = UserIds::lookup(&name);

        Self { name, ids }
    }
}

impl User {
    pub fn switch(&mut self) -> Result<(), UserSwitchError> {
        let ids = self.ids?;
        if unsafe { libc::setresgid(ids.gid, ids.gid, ids.gid) != 0 } {
            let e = errno();
            let c: i32 = e.into();
            if c == EPERM as i32 {
                return Err(UserSwitchError::NotPermitted);
            }
            return Err(UserSwitchError::Error("setresgid()", e));
        }

        // TODO: supplemental groups

        if unsafe { libc::setresuid(ids.uid, ids.uid, ids.uid) != 0 } {
            let e = errno();
            let c: i32 = e.into();
            if c == EPERM as i32 {
                return Err(UserSwitchError::NotPermitted);
            }
            return Err(UserSwitchError::Error("setresuid()", e));
        }

        // TODO: getresuid/getreguid and verify change is applied
        // Optional: try setting back to previous user and verify failure

        Ok(())
    }
}
