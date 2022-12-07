fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 4, 5];

    // Use iter() to iterate through every number, perform some map fn, then collect
    let plus_one: Vec<i32> = numbers.iter().map(|num| num + 1).collect();
    dbg!(plus_one);

    // Use iter() to filter out elements that match some criteria
    let new_numbers: Vec<_> = numbers.iter().filter(|num| **num > 3).collect();
    dbg!(new_numbers);

    // Use iter() to find element that match some criteria\
    // Return none if not found
    let find_me: Option<_> = numbers.iter().find(|num| **num > 23);
    dbg!(find_me);
}
