#[allow(unused)]

fn main() {
    let number = 10;

    if number % 10 == 0 {
        println!("The number is divisible by 10");
    } else if number % 5 == 0 {
        println!("The number is divisible by 5");
    } else if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("Number is not divisible by 10, 5, 4, or 2");
    }

    // assign

    let condition = true;

    let number = if condition { 10 } else { 5 }; // the if and else must return the same value, the
                                                 // code below is invalid:
                                                 // let number = if condition { 10 } else { 'a' };

    // loop

    // loop {
    //     println!("infinit loop");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter + 100;
        }
    };

    println!("Result is {}", result);

    // label a loop

    let mut rows = 6;
    let mut cols = 5;

    'rows: loop {
        print!("#{}->", rows);

        rows -= 1;

        'cols: loop {
            print!("(col:{})", cols);
            cols -= 1;

            if cols == 3 {
                break 'rows;
            }
        }
    }

    println!();

    // while

    let mut counter = 5;

    while(counter > 0) {
        println!("{}", counter);
        counter -= 1;
    }

    // for
    // - safer than using an index for arrays

    let arr = [10, 20, 30, 40];

    for number in arr {
        println!("{}", number);
    }

    // countdown
    for number in (1..=5).rev() {
        println!("{number}!");
    }

    println!("done");
}
