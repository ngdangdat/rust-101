use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let lower: u8 = 1;
    let upper: u8 = 11;

    println!("Please input your guessing number, range from {} to {}", lower, upper);
    let secret_number: u8 = rand::thread_rng().gen_range(lower, upper);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                println!("Err: {}", _err);
                println!("Input an integer number range from {} to {}, you arsehole!", lower, upper);
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    
}

