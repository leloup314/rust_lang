use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number, me lad!");
    
    // Generate a random integer between 0 and 100
    let number = rand::thread_rng().gen_range(0..=100);
    
    loop {

        println!("What number will you guess, my guy?");

        let mut current_guess = String::new();

        io::stdin()
            .read_line(&mut current_guess)
            .expect("Could not read line!");

        let current_guess: u32 = match current_guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {println!("Please input an integer"); continue;}
            };

        match current_guess.cmp(&number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("Oi, you cheeky wanker got it right!"); break;}
        }

        //println!("The random number is {}", number);

    }
}
