fn main() {
    let mut content = String::from("content string");

    append_to_string(&mut content);
}

fn append_to_string(string: &mut String) {
    string.push_str("; and updated");
}
