struct Point 
{
    x: i32,
    y: i32,
}

enum Message
{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum color
{
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2
{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn foo(_: i32, y: i32)
{
    println!("Only need y parameter: {y}");
}

fn main() 
{
    // Matching literals
    let x = 1;
    match x 
    {
        1 => println!("one"),
        2 => println!("two"),
        _ -> println!("ANYTHING"),
    }

    // Matching named variables
    // Match expression shadows variable y
    let x = Some(5);
    let y = 10;
    match x 
    {
        Some(50) => println!("Got 50"),
        // a new y variable. It matches any value inside a Some, which is the x
        // it becomes y = 5
        Some(y) => println!("Matched, y = {y}"),    
        _ => println!("Default case x = {x:?}"),
    }
    // prints x = Some(5) and y = 10
    println!("At the end: x = {x:?}, y = {y}");

    // Multiple patterns
    let x = 1;
    match x 
    {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges of values
    let x = 5;
    match x 
    {
        1..=5 => println!("one through five"),
        _ => println!("Something else"),
    }
    let x = 'c';
    match x
    {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring structs
    let p = Point{x: 0, y: 7};
    let Point {x: a, y: b} = p;
    // let Point {x, y} = p; shorthand
    assert_eq!(0, a);
    assert_eq!(7, b);
    
    // Destructuring and matching literal values 
    let p = Point{x: 0, y: 7};
    match p
    {
        Point {x, y: 0} => println!("On x axis at {x}"),
        Point {x: 0, y} => println!("On y axis at {y}"),
        Point {x, y} => println!("On neither axis: {x}, {y}");
    }

    // Destructuring enum
    let msg = Message::ChangeColor(0, 160, 255);
    match msg
    {
        Message::Quit => {println!("Quit has no data to destructure")};
        Message::Move {x, y} => {println!("Move in x: {x} and y: {y} direction")};
        Message::Write(text) => {println!("Text message: {text}")};
        Message::ChangeColor(r, g, b) => {println!("Change color to rgb: {r}, {g}, {b}")};
    }

    // Destructuring Nested Structs and Enums
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg 
    {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {println!"RGB: {r}, {g}, {b}"};
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {println!"HSV: {h}, {s}, {v}"};
        _ => (),
    }

    // Destructuring Structs and Tuples
    // let ((feet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});

    // Ignoreing values with _
    foo(3, 4);  // Ignores value 3.

    let mut set_value = Some(5);
    let new_set_value = Some(10);
    match (set_value, new_set_value)
    {
        (Some(_), Some(_)) => {println!("Cannot overwrite existing customized value")};
        _ => {set_value = new_set_value};
    }
    // Prints Can't overwrite existing...
    // then prints setting is Some(5)
    println!("setting: {set_value}");

    let numbers = (2, 4, 8, 16, 32);
    match numbers
    {   // Only prints 2 8 32
        (a, _, c, _, d) => {println!("Numbers: {a} {c} {d}")};
    }

    // Ignore unused variable with _
    let _x = 5; // No warning
    let y = 10; // Gets warning 

    // Causes Error
    // Unused variable can take ownership
    let s = Some(String::from("Hello"));
    if let Some(_s) = s // change _s to _ to make it work
    {
        println!("found a string");
    }
    println!("{s:?}");  // the string is gone. unable to print

    // Ignoring values with ..
    let origin = Point {x: 0, y: 0};
    match origin
    {
        Point {x, ..} => println!("x is {x}"),
    }

    let numbers = {2, 4, 8, 16, 32};
    match numbers
    {
        // (.., second, ..) is ambiguous. only one .. allowed
        (a, .., e) =>   // gets the first and last number
        {
            println!("Some numbers: {a}, {e}");
        }
    }

    // Extra conditionals with matching guards
    // additional if condition specified after pattern in match arm

    let num = Some(4);
    match num
    {   // Prints: number 4 is even
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x 
    {   // prints default case
        Some(50) => println("Got 50"),

        // the y is from the outer scope
        // because if n == y is not a pattern. Doesnt introduce new variables
        Some(n) if n == y => println!("Matched, n = {n}"),  
        _ => println!("Default case x = {x:?}"),
    }
    println!("at the end: x {x:?}, y {y}");

    let x = 4;
    let y = false;
    match x
    {   // arm only matches if x is equal to 4, 5, or 6 and if y is true
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ lets us create a variable that holds a value 
    // at the same time as we're testing that value for a pattern match
    // Lets us test a value and save it in a variable within one pattern
    
    enum Message1
    {
        Hello { id: 32},
    }
    let msg = Message::Hello {id: 5};
    match msg
    {
        Message::Hello {id: id_variable @ 3..=7} => 
        {
            println!("Found an id in range: {id_variable}"),
        }
        Message::Hello {id: 10..=12} =>
        {
            println!("Found an id in another range")
        }
        Message::Hello {id} => println!("Found some other id {id}"),
    } 
    // Prints found an id in range: 5.
    // Specifying id_variable @ before range 3..=7:
    // Captures any value matched the range while also testing that the value matched the range pattern

    // second arm: only have range specified in pattern.
    // doesnt have actual value of id field. Couldve been 10 11 or 12 but doesnt know which it is
    
    // third arm: any value matches pattern as no test is applied to id
}

