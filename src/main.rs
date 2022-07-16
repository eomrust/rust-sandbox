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

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Handling Multiple Conditions with else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");


    // Safer loops relative to conditional loops
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // For loops
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn chapter_three_examples(){
    // 3.1
    variables_mutability();

    // 3.2 Data Types
    data_types();
    numerical_operations();
    invalid_array();

    // 3.3 Functions
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {x}");
    let x = plus_one(5);
    println!("The value of x is: {x}");

    // 3.4
    control_flow();
}

fn main() {
    hello_world();
    println!("{}", THREE_HOURS_IN_SECONDS);

    chapter_three_examples();
}
