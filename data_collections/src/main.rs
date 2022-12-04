// Implementation functionality

struct Temperature {
    degrees_f: f64,
}

// Collection of functions that belong to a struct
impl Temperature {
    // fn show_temp(temp: Temperature) {
    //     println!("{} degrees F", temp.degrees_f)
    // }

    // Use self to refer to struct as a shorthand
    fn show_temp(&self) {
        println!("{} degrees F", self.degrees_f)
    }

    // Returning self as an object
    fn freezing() -> Self {
        Self { degrees_f: 2.2 }
    }
}

fn main() {
    // Call the functions that struct owns
    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
}
