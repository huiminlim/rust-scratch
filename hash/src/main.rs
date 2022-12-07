use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut people = HashMap::new();
    people.insert("Susan", 20);
    people.insert("Ed", 12);
    people.remove("Susan");

    // Searching in Hashmaps
    match people.get("Ed") {
        Some(age) => println!("Hello Ed is {:?}", age),
        None => println!("Not found"),
    }

    // Iterating Hashmaps
    for (person, age) in people.iter() {
        println!("person {:?} age{:?}", person, age);
    }

    // Looping keys and values
    for person in people.keys() {
        println!("Person {:?}", person);
    }
    for age in people.values() {
        println!("Age {:?}", age);
    }
}
