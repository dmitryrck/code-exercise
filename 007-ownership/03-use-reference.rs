fn main() {
    let string1 = String::from("Hello");

    let length = calculate_length(&string1);
    let length2 = calculate_length(&string1);

    println!(
        "The length of '{}' is '{}' and again '{}'",
        string1, length, length2
    );
}

fn calculate_length(string: &String) -> usize {
    string.len()
}
