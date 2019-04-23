use errno::{errno, Errno};
use libc::c_int;

pub fn enter() -> Result<(), Errno> {
    if unsafe { cap_enter() == 0 } {
        Ok(())
    } else {
        // ENOSYS
        Err(errno())
    }
}

pub fn sandboxed() -> bool {
    unsafe { cap_sandboxed() == 1 }
}

#[link(name = "c")]
extern "C" {
    fn cap_enter() -> c_int;
    fn cap_sandboxed() -> c_int;
}
