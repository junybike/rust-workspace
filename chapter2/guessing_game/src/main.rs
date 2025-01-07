use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret}");
    
    loop
    {
        println!("Your guess: ");

        // guess is a mutable variable holding a new, empty instance of a String value 
        let mut guess = String::new();   

        // Handling user input
        io::stdin()
            // take whatever user types into std input and append that into a string (no overwriting its contents)
            // &: reference. No needing to copy 'guess' data into memory multiple times   
            .read_line(&mut guess)      
            .expect("Failed to read");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;}
        }
    }
}
