fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

#[cfg(test)]
mod test {

    // Access all codes in above
    use crate::*;

    #[test]
    fn check_all_caps() {
        // If assert failes, then fail test
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "All char should be in uppercase");
    }
}

fn main() {
    println!("{:?}", all_caps("Hello, world!"));
}
