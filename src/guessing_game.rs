use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn easter_eggs(guess: u32, games_played: u32) {
    // https://www.dictionary.com/e/slang/42/
    if guess == 42 && games_played == 1 {
        println!(
            "bonus points for knowing the ultimate question of life, the universe, and everything"
        )
    }

    if games_played == 7 {
        println!("Have you considered playing something else?")
    }
}
pub fn play() {
    let mut games_played: u32 = 0;
    loop {
        games_played += 1;

        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess: String = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        easter_eggs(guess, games_played);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("guess: {guess} secret_number: {secret_number}");
    }
}
