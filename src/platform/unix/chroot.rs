use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use derive_more::Display;
use errno::{errno, Errno};
use libc::{self, EPERM};

#[derive(Debug, Display, Clone, Copy)]
pub enum ChrootError {
    #[display(fmt = "Path not found")]
    NotFound,
    #[display(fmt = "Insufficient privileges")]
    NotPermitted,
    #[display(fmt = "{} error: {}", "_0", "_1")]
    Error(&'static str, Errno),
}

impl std::error::Error for ChrootError {}

#[derive(Debug, Display)]
#[display(fmt = "{}", "path.display()")]
pub struct Chroot {
    pub path: PathBuf,
}

impl Default for Chroot {
    fn default() -> Self {
        Self::from("/var/empty")
    }
}

impl<'a, T: AsRef<Path>> From<T> for Chroot {
    fn from(path: T) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}

impl Chroot {
    pub fn apply(&self) -> Result<(), ChrootError> {
        // TODO: consider clearing locale etc

        // Paths containing NULL bytes cannot exist.
        let path =
            CString::new(self.path.as_os_str().as_bytes()).map_err(|_| ChrootError::NotFound)?;

        if unsafe { libc::chroot(path.as_ptr()) != 0 } {
            let e = errno();
            if e.0 == EPERM as i32 {
                return Err(ChrootError::NotPermitted);
            }
            return Err(ChrootError::Error("chroot()", e));
        }

        std::env::set_current_dir("/")
            .map_err(|e| ChrootError::Error("chdir()", Errno(e.raw_os_error().unwrap_or_default())))
    }
}
