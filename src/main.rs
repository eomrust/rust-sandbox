#![allow(unused)]

use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn hello_world() {
    println!("Hello, world!");
}

fn variables_mutability() {
    let x = 5;
    println!("The value of x is: {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   ";
    println!("Length of spaces {}", spaces.len());
}

fn invalid_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index between 0-{}", a.len() - 1);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn data_types() {
    let guess: i16 = 1_900i16;
    println!("Guess: {guess}");

    let y: f32 = 3.0; // f32
    println!("floating point: {y}");

    let b: bool = false; // with explicit type annotation
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("bool: {b} char: {c}, {z}, {heart_eyed_cat}");

    /*
    tuples
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {x} y: {y} z:{z}");
    let x: i32 = tup.0;
    let y: f64 = tup.1;
    let z: u8 = tup.2;
    println!("x: {x} y: {y} z:{z}");

    /*
    arrays
    */
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];

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
    println!("january - ({})", &months[0]);
}

fn numerical_operations() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;
}

fn main() {
    hello_world();

    // 3.1
    variables_mutability();

    // 3.2
    data_types();
    numerical_operations();
    invalid_array();

    // 3.2

    println!("{}", THREE_HOURS_IN_SECONDS);
}
