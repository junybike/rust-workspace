fn main() 
{
    // If user specifies a fav color, uses it as background
    // If no fav color but today is tuesday, bg is green
    // Else, if user specified age as string, color is purple or orange
    // If nothing, uses blue for bg

    let fav_color: Option<&str> = None;
    let is_tues = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_col
    {
        println!("Using fav color {color} as background");
    }    
    else if is_tues
    {
        println!("Tuesday is green day");
    }
    else if let Ok(age) = age
    {
        if age> 30 
        {
            println!("Using purple as background color");
        }
        else
        {
            println!("Using orange as background color");
        }
    }
    else
    {
        println!("bg is blue");
    }

    // print values as long as stack.pop() returns Some
    // pop returns None is vector is empty
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop()
    {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate()
    {
        println!("{value} is at index {index}");
    }

    let x = 5; // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    cord(&point);
}

fn foo(x: i32)  // x is the pattern.
{
    // implementation
}

fn cord(&(x, y): &(i32, i32))
{
    println!("current location: {x}, {y}");
}