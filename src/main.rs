use unprivileged::{capsicum, unix};

fn main() {
    let u = unix::User::from("freaky");
    let d = unix::Chroot::from("/var/empty");
    match d.apply() {
        Ok(()) => {
            println!("Chrooted to {}", d);
        }
        Err(e) => {
            println!("Failed chroot to {}: {:?}", d, e);
        }
    }

    match u.switch() {
        Ok(()) => {
            println!("Switched to {}", u);
        }
        Err(e) => {
            println!("Failed switch to {}: {:?}", u, e);
        }
    }

    println!("Capsicum sandbox: {}", capsicum::sandboxed());
    println!("Capsicum enter: {:?}", capsicum::enter());
    println!("Capsicum sandbox: {}", capsicum::sandboxed());
}
