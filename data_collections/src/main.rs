// Implementation functionality

struct Temperature {
    degrees_f: f64,
}

// Collection of functions that belong to a struct
impl Temperature {
    fn show_temp(temp: Temperature) {
        println!("{} degrees F", temp.degrees_f)
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };

    // Call the functions that struct owns
    Temperature::show_temp(hot);
}
