#[derive(Debug)]

struct Rectangle
{
    width: u32,
    height: u32,
}

fn main() 
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50,
    };

    println!("area of rect1 is {}", area(&rect1));  
    println!("Rect1 is {rect1:#?}");                 // debug info of rect1

    let scale = 2;
    let rect1 = Rectangle
    {
        width: gdb!(30 * scale),    // dgb! returns the ownership of expression's value. Therefore, width field would still be the same as without gdb!  
        height: 50,
    };
                    
    dgb!(&rect1);   // dgb! can take ownership. Therefore, use reference.
                
}

fn area(rectangle: &Rectangle) -> u32
{
    rectangle.width * rectangle.height
}
