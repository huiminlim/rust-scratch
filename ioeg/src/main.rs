// I/O library
use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?; // ? means function may fail
    Ok(buffer.trim().to_owned())
}

fn main() {
    println!("Hello, world!");

    let mut all_input = vec![];
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
            }
            Err(err) => {
                println!("Error! {:?}", err);
            }
        }
        times_input += 1
    }

    for input in all_input {
        println!(
            "Original: {:?}, capitalized: {:?}",
            input,
            input.to_uppercase()
        );
    }
}
