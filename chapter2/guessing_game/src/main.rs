use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("Guess the number!");
    
    // Secret number. Has a number type (1 ~ 100)
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
            .expect("Failed to read");  // expect crashes the program and prints that message

        // Check if the input guess is an integer. Trim eliminates '\n'. Parse converts a string to another type and returns either 'Ok' or 'Err'
        // match handles errors. Has 2 arms here: Ok and Err.
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,     // Input is an integer
            Err(_) => continue, // Input is not an integer. Receive a new input
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret)    // The cmp function returns one of the 3 value: Ordering::Less, Ordering::Greater, Ordering::Equal.
        {
            Ordering::Less => println!("Too small!"),           // Case: the guess is less than the secret number
            Ordering::Greater => println!("Too big!"),          // Case: the guess is larger than the secret number
            Ordering::Equal => {println!("You win!"); break;}   // Case: the guess is correct. End the loop
        }
    }
}
