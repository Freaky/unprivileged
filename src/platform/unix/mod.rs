mod chroot;
mod user;

pub use chroot::Chroot;
pub use user::User;

use derive_more::Display;

#[derive(Default, Debug, Display)]
#[display(
    fmt = "User({}) Chroot({})",
    "user.as_ref().map(User::to_string).unwrap_or_default()",
    "chroot.as_ref().map(Chroot::to_string).unwrap_or_default()"
)]
pub struct PrivDrop {
    pub user: Option<User>,
    pub chroot: Option<Chroot>,
}

impl PrivDrop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user<U: Into<User>>(&mut self, user: U) -> &mut Self {
        self.user = Some(user.into());
        self
    }

    pub fn chroot<C: Into<Chroot>>(&mut self, chroot: C) -> &mut Self {
        self.chroot = Some(chroot.into());
        self
    }

    pub fn apply(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(chroot) = &self.chroot {
            chroot.apply()?;
        }

        if let Some(user) = &self.user {
            user.apply()?;
        }

        Ok(())
    }
}
