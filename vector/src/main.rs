fn main() {
    println!("Hello, world!");

    // Immutable vector
    let numbers = vec![1, 2, 3];
    for num in numbers {
        println!("{}", num);
    }

    // Mutable vector
    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(1);
    my_numbers.push(1);
    my_numbers.push(1);
    my_numbers.pop();
    my_numbers.len();
}
