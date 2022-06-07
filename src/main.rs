use systemstat::{Duration, Platform, System};
fn main() {
    println!("Username: {}", whoami::username());
    println!("System: {}", whoami::distro());

    let sys = System::new();
}
