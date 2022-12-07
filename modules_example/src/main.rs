mod greet {
    pub fn hello() {
        println!("hello");
    }
}

fn main() {
    println!("Hello, world!");
    greet::hello();
}
