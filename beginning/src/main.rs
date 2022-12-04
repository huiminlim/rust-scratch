fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn main() {
    println!("Hello, world!");

    let sum = 2 + 1;
    let value = sub(10, 2);

    println!("Hello, {} {}", sum, value);

    // Match is similar to switch
    let some_bool = true;
    match some_bool {
        true => println!("True"),
        false => println!("False"),
    };
}
