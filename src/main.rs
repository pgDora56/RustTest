use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let sec_n = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Enter your number: ");

        let mut g = String::new();
        io::stdin().read_line(&mut g).expect("Failed to read line");
        let g: u32 = match g.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value, please retype number.");
                continue;
            }
        };

        println!("You guessed: {}", g);

        match g.cmp(&sec_n) {
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
            Ordering::Less => println!("Small..."),
            Ordering::Greater => println!("Big..."),
        }
    }
}

// https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html
