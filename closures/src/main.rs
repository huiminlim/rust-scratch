fn main() {
    println!("Hello, world!");

    // CLosures
    let add1 = |a: i32, b: i32| -> i32 { a + b };
    let add2 = |a, b| a + b;
    let sum1 = add1(1, 2);
    let sum2 = add2(1, 2);
    println!("Sum: {:?}", sum1);
    println!("Sum: {:?}", sum2);
}
