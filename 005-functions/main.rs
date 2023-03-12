#![allow(unused)]

fn five() -> i32 {
    5
}

fn main() {
    let y = 6; // statement
    let number_five = five();

    println!("Hello from main");

    println!("Number {}+1 = {}", number_five, plus_one(number_five));

    hello_function(5);
}

// 1-Order does not matter
// 2-Type for argument is mandatory
fn hello_function(number: i32) {
    println!("Hello from function, number: {}", number);
}

fn plus_one(number: i32) -> i32 {
    number + 1
}
