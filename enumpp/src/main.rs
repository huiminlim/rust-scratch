#[allow(dead_code)]
enum Discount {
    Percent(i32),
    Flat(i32),
}

fn main() {
    println!("Hello, world!");

    let n = 3;
    match n {
        3 => println!("Three"),
        other => println!("{:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("2"),
        Discount::Flat(_) => println!("Any value"),
        _ => println!("Percent"),
    }
}
