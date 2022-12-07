fn main() {
    println!("Hello, world!");

    fn maybe_num() -> Option<i32> {
        Some(1)
    }

    let plus_one = maybe_num().map(|num| num + 1);
    println!("Number is {:?}", plus_one);
}
