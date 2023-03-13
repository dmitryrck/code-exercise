#[allow(unused)]

fn main() {
    // String has 3 parts:
    // 1-pointer: where to memory it is pointing to
    // 2-length: current length of this string
    // 3-capacity: maximum the string can grow, e. g., how much the allocator has take from memory
    let str = String::from("This is a string");
    let mut mut_str = String::from(str);

    mut_str.push_str(", and this is a change");

    println!("Final: {}", mut_str);

    // copies, now both number5 and number5_again has the number 5
    let number5 = 5;
    let number5_again = number5;

    // string, reference
    let string1 = String::from("content");
    let string2 = string1;

    // when we assign a String to another rust copies the 1-pointer, 2-length, and 3-capacity, but
    // not the content in the heap
    //
    // XXX
    //
    // in the example above string1 is no longer valid, and it is not possible to use it

    let string1 = String::from("content again");
    let string2 = string1.clone(); // allocates memory in the heap and copies the string
}
