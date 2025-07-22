use std::io;
use rand::Rng; // Rng is a trait
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // thread_rng is local to the current thread of exec and seeded by OS
                                                                // inclusive on upper and lower bounds
    // println!("The secret number is: {secret_number}");

    loop { // infinite loop
        println!("Please input your guess:");

        let mut guess = String::new(); // mut for mutable, allows changing

        io::stdin() // returns Result: Ok/Err (similar to an HTTP response's status)
            .read_line(&mut guess)  // & also indicates reference (allows for single memory destination access)
                                    // also need to write &mut instead of just & to make refs mutable
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // reuses old variable instead of creating a new one (shadowing)
            Ok(num) => num,
            Err(_) => continue
        };
        // match: moves from [crashing] on error to [handling] the error
        // trim: eliminates whitespace
        // parse: converts string to some other type


        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // can also call functions!
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
