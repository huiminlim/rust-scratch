fn main() {
    println!("Hello, world!");

    struct Customer {
        age: Option<i32>,
        email: String,
    }

    let mark = Customer {
        age: Some(22),
        email: "hello@mark.com".to_owned(),
    };
    let becky = Customer {
        age: None,
        email: "becky@ehllo.com".to_owned(),
    };

    match mark.age {
        Some(age) => println!("Age is {:?}", age),
        None => println!("Not provided"),
    };

    match becky.age {
        Some(age) => println!("Age is {:?}", age),
        None => println!("Not provided"),
    };

    println!("{:?} {:?}", mark.email, becky.email);
}
