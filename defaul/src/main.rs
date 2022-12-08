struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

// Implement defaults as some
// crates tend to call default functions
// when creating the struct
impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

fn main() {
    println!("Hello, world!");

    let p1 = Package::default();
    println!("{:?}", p1.weight);

    let p2 = Package::new(3.4);
    println!("{:?}", p2.weight);
}
