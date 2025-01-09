#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

// Associated functions for Rectangles
impl Rectangle  // everything within this block will be associated with Rectangle type
{
    fn area(&self) -> u32       // If we wanted to change some values, we would use "&mut self"
    {
        self.width * self.height
    }
    fn width(&self) -> bool
    {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self    // Constructor. associated function that arent methods
    {
        Self
        {
            width: size,
            height: size,
        }
    }
}

impl Rectangle
{
    // able to have multiple impl blocks for structs
}

fn main() 
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50,
    };

    if rect1.width()
    {
        println!("rectangle width has to be nonzero");
    }

    println!("Area of rec1: {}", rect1.area());

    // The two are the same
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    let rect2 = Rectangle::square(3);
}
