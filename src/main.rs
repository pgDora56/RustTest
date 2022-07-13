use rand::Rng;
use std::io;

fn main() {
    println!("Enter your number: ");

    let sec_n = rand::thread_rng().gen_range(1..101);

    let mut g = String::new();

    io::stdin().read_line(&mut g).expect("Failed to read line");

    println!("You guessed: {}", g)
}

// https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html
