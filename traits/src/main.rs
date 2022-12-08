// Traits are a collection of functions
// that can be implemented
// like an interface class
trait Noise {
    fn make_noise(&self);
}

// Implementing Noise trait for Person struct
struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("Hello");
    }
}

// Implementing Noise trait for Dog
struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("Woof");
    }
}

fn main() {
    println!("Hello, world!");

    let p = Person {};
    p.make_noise();

    let d = Dog {};
    d.make_noise();
}
