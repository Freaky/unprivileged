# unprivileged

Privilege dropping for Rust.

Currently supports Unix user/group changes (including supplementary groups),
and chroot, with vague aspirations for more.

```rust
use unprivileged::unix::{User, Chroot, PrivDrop};

let user = User::from("nobody"); // also User::default();
let chroot = Chroot::from("/var/empty"); // also Chroot::default();

user.apply()?;
chroot.apply()?;

// or...
let mut priv = PrivDrop::new();
priv.user("nobody")
    .chroot("/var/empty");

priv.apply()?;
```

## See Also

Crates actually worth using at this point:

rusty-sandbox, privdrop, clap-permission-flag, capsicum, pledge.
