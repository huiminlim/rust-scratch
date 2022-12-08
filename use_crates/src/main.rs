use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let d = Duration::from_secs(9876);
    println!("{}", d.as_micros());
}
