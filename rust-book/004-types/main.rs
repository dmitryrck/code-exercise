#![allow(unused)]

fn main() {
    let float_2 = 2.0; // f64 by default
    let float_3: f32 = 3.0; // force f32

    let false_var = false;
    let true_var: bool = true;

    let char_a = 'a';
    let neutral_face = '😐';

    let tup_1: (i32, f64, u8) = (500, 6.4, 1);
    let tup_2 = (500, 6.4, 1);
    let (tup_2_x, tup_2_y, tup_2_z) = tup_2;
    let five_hundred = tup_2.0;

    // Array has fixed size
    // "An array is a single chunk of memory of a known, fixed size that can be allocated on the stack"
    let arr = [1, 2, 3, 4];
    let five_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let five_of_three = [3; 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Hello");

    return;
}
