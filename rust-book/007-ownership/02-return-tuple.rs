use std::convert::TryInto; // the compiler told me so

fn main() {
    let s1 = String::from("create in the main");
    let (s1, len) = calculate_length(s1);

    println!("length of '{}' is '{}'", s1, len);
}

fn calculate_length(s1: String) -> (String, i32) {
    let length: i32 = s1.len().try_into().unwrap(); // .try_into().unwrap() was the compiler

    return (s1, length);
}
