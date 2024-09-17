use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Number guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("enter your number ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read inputs");

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid input");
                continue;
            }
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you won");
                break;
            }

            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
        }
    }
}
