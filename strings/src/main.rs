fn main() {
    println!("Hello, world!");

    fn print(data: &str) {
        println!("{}", data);
    }

    print("Hello!");
    let own_string = "own_string".to_owned();
    let another_owned = String::from("owned");
    print(&own_string);
    print(&another_owned);
}
