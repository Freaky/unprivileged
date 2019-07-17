use unprivileged::unix::PrivDrop;

fn main() {
    let mut privs = PrivDrop::new();
    privs
        .user("nobody")
        .chroot("/var/empty");

    println!("{}", privs);
    dbg!(&privs);

    if let Err(e) = privs.apply() {
        eprintln!("PrivDrop failed: {}", e);
    }
}
