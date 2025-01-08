use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() 
{
    let a = [1, 2, 3, 4, 5];
    println!("Enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered is not a number");
    
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
    return;

    //
    // 3.1
    //
    let x = 5;
    println!("The value of x: {x}");
    
    // Causes error!
    // x = 6;
    // println!("The value of x: {x}");

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("Value of y in the inner scope: {y}");
    }

    println!("Value of y: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("amount of spaces: {spaces}");

    // CAUSES ERROR!
    // let mut mspaces = "   ";
    // mspaces = mspaces.len();
    
    //
    // 3.2
    //

    // u32 unsigned integer
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess: {guess}");

    let fx = 2.0;       // f64 float
    let fy: f32 = 3.0;  // f32 float

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("value of b is: {b}");
    
    // accessing elements in tuple
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;

    // array
    let arr1 = [1, 2, 3, 4, 5];             
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];   // type; amount of elements
    let arr3 = [3; 5];                      // Initialization only

    // accessing array elements
    let arr1_0 = arr1[0];
    let arr1_1 = arr1[1];
}
