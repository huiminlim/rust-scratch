#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");

    let a: Option<i32> = Some(1);
    let a_is_some = a.is_some(); // has data
    let a_is_none = a.is_none(); // has no data

    // No need to check if data is available
    // Only applies if there is data
    let a_mapped = a.map(|num| num + 1);

    // Filter the numbers
    // Return number if true, else return None
    let a_filtered = a.filter(|num| num > &10);

    // Set to some value if not found
    let a_or_else = a.or_else(|| Some(5));
}
