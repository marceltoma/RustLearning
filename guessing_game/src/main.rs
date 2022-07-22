use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret:u16 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret}");

    loop {
        println!("please input your guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line!");

        let u_guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{guess} is not a valid number.");
                continue;
            }
        };

        match u_guess.cmp(&secret) {
            Ordering::Less => print!("too small!!"),
            Ordering::Greater => println!("too big!!"),
            Ordering::Equal => {
                println!("you guessed it, it's {guess}");
                break;
            },
        }
    }
}

