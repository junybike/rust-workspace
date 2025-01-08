fn main() 
{
    println!("Hello, world!");
    message();
    view(5);
    measure(100, 'm');

    // CAUSES ERROR! let statements does not return a value
    // let y = 6;
    // let x = (let y = 6);

    let y = 
    {
        let x = 3;
        x + 1       // evaluates to 4. No semicolon. (Semicolon at the end = statement)
    };
    println!("Value of y: {y}");

    let x = five();
    println!("Value of x: {x}");
    let x = plus(3);
    println!("Value of x: {x}");
}

fn message()
{
    println!("Goodbye, world..");
}

fn view(x: i32)
{
    println!("value of x: {x}");
}

fn measure(value: i32, unit: char)
{
    println!("measurement: {value}{unit}");
}

fn five() -> i32
{
    5
}

fn plus(x: i32) -> i32
{
    x + 1
}