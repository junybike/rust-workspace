fn main() 
{
    let number = 3;

    if number < 5 
    {
        println!("TRUE");
    }
    else
    {
        println!("FALSE");
    }

    // CAUSES ERROR! Rust does not automatically convert integer to bools
    // if number {println!("number was 3")};

    if number != 0 {println!("number is not a zero");}
    
    let number = 6;

    if number % 4 == 0 {println!("number is divisible by 4");} 
    else if number % 3 == 0 {println!("number is divisible by 3");} 
    else if number % 2 == 0 {println!("number is divisible by 2");} 
    else {println!("number is not divisible by 4, 3, or 2");}

    let condition = true;
    let number = if condition {5} else {6}; // condition types must be the same
    println!("value of number: {number}");

    // Loop
    let mut number = 0;
    loop
    {
        println!("hello");
        number = number + 1;
        if number > 5 {break};
    }

    // Assign value with Loop 
    let mut number = 0;
    let result = 
    loop
    {
        number += 1;
        if number == 5 {break number * 2;}
    };  // Assign the value in 'result'
    println!("Result: {result}");

    // Loop labels
    let mut count = 0;
    'counting_up: loop 
    {
        println!("count = {count}");
        let mut remaining = 10;

        loop 
        {
            println!("remaining = {remaining}");
            if remaining == 9 {break;}
            if count == 2 {break 'counting_up;} // Exits the outer loop
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loop
    let mut number = 3;

    while number != 0
    {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!");

    // arr value iteration
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5
    {
        println!("value is {}", arr[index]);
        index += 1;
    }

    for element in arr
    {
        println!("value is {element}");
    }

    // range for loop
    // rev() reverses the order
    for number in (1..4).rev()
    {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}
