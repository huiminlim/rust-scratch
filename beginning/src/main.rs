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

    let some_int = 3;
    match some_int {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("Others"), // Everything default case
    };

    // Loop syntax
    let mut counter = 0; // to change value of variable, use `mut` keyword
    loop {
        counter += 1;
        println!("Hello! - {} ", counter);
        if counter == 6 {
            break;
        }
    }

    // While loop syntax
    let mut value = 0;
    while value < 12 {
        println!("While loop! - {} ", value);
        value += 1;
    }

    // Enums
    enum Direction {
        // Up,
        // Down,
        Left,
        Right,
    }
    let mut direction = Direction::Left;
    // Syntax to check for enum equality
    if matches!(direction, Direction::Left) {
        println!("First check");
    }
    // direction = Direction::Up; // Appease deadcode warning
    // direction = Direction::Down; // Appease deadcode warning
    direction = Direction::Right; // Appease deadcode warning
    match direction {
        Direction::Left => println!("Left"),
        // Direction::Down => println!("Down"),
        Direction::Right => println!("Right"),
        // Direction::Up => println!("Up"),
    }
}
