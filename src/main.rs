use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Rust Guessing Game");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {secret_number}");

        println!("Enter your number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You Guessed: {}", guess);

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
