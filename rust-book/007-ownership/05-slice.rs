fn main() {
    let mut content = String::from("this is a multiple word");

    let first_word_length = first_word_index(&content);

    let first_word = &content[0..first_word_length];

    //content.clear(); // This cannot happen because first_words "points" (?) to parts of content

    println!("First word of '{}' is '{}'", content, first_word);

    content.clear();

    // part 2
    let mut numbers = String::from("one two three");
    let first_slice = first_word_slice(&numbers);

    println!("First word of '{}' is '{}'", numbers, first_slice);

    numbers.clear();

    // part 3
    let mut numbers = String::from("four five six");
    let second_slice = second_word_slice(&numbers);

    println!("Second word of '{}' is '{}'", numbers, second_slice);

    numbers.clear();

    // part 4
    let mut numbers = String::from("seven eight nine");
    let second_slice = String::from(second_word_slice(&numbers));
    numbers.clear();

    println!("Second word of '{}' is '{}'", numbers, second_slice); // Here the original string is
                                                                    // empty
                                                                    //
    // part 5
    let numbers = String::from("ten eleven twelve");
    let first_slice = first_word_slice_from_str(&numbers);
    println!("First word of '{}' is '{}'", numbers, first_slice);
}

fn first_word_index(content: &String) -> usize { // A more idiomatic rust code is `content: &str`
    let bytes = content.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    content.len()
}

fn first_word_slice(content: &String) -> &str { // A more idiomatic rust code is `content: &str`
    let bytes = content.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &content[..index];
        }
    }

    &content[..]
}

fn first_word_slice_from_str(content: &str) -> &str {
    let bytes = content.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &content[..index];
        }
    }

    &content[..]
}

fn second_word_slice(content: &String) -> &str {
    let mut first_index = -1;
    let mut second_index = -1;

    let bytes = content.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_index > 0 {
            second_index = index as i32;
            break
        }

        if item == b' ' && first_index == -1 {
            first_index = (index + 1) as i32;
        }
    }

    if first_index == -1 { first_index = 0; }
    if second_index == -1 { second_index = content.len() as i32; }

    let first_index = first_index as usize;
    let second_index = second_index as usize;

    &content[first_index..second_index]
}
