use std::io;

fn main() {
    println!("Fizzbuzz Max:");

    let mut maxi = String::new();
    io::stdin()
        .read_line(&mut maxi)
        .expect("Failed to read line");

    let maxi: usize = maxi
        .trim()
        .parse()
        .expect("MaxNumber entered was not a number");

    let mut i = 0;
    while i <= maxi {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("Num: {}", i)
        }
        i += 1;
    }
}
