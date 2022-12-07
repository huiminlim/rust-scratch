fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 4, 5];

    // Use iter() to iterate through every number, perform some map fn, then collect
    let plus_one: Vec<i32> = numbers.iter().map(|num| num + 1).collect();

    dbg!(plus_one);
}
