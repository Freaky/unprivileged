mod chroot;
mod user;

pub use chroot::Chroot;
pub use user::User;

use derive_more::Display;
use errno::{errno, Errno};

#[derive(Default, Debug)]
pub struct UnixPrivs {
    user: Option<User>,
    chroot: Option<Chroot>,
}

// impl UnixPrivs {
//     fn drop() -> Result<(), Box<dyn std::error::Error>> {

//     }
// }
