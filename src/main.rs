#![allow(unused)]
mod chapter_three;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    hello_world();
    println!("{}", THREE_HOURS_IN_SECONDS);

    chapter_three::examples();
}
