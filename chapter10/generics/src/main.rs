// Instead of creating two functions for finding largest int and finding largest char,
// define a function with generic
fn largest<T>(list: &[T]) -> &T     // this function is generic over some thpe T
{
    let mut largest = &list[0];

    for item in list
    {
        if item > largest
        {
            largest = item;
        }
    }
    largest
}

// Struct using generic type parameter. Uses any type to hold x and y cord 
struct Point<T>
{
    x: T,
    y: T,
}
struct Point<X1, Y1>
{
    x: X1,
    y: Y1,
}
//
impl<T> Point<T>
{
    fn x(&self) -> &T   // Returns a reference to the data in field x for Point
    {
        &self.x
    }
}
// This impl only applies when Point holds f32 type
impl Point<f32>
{
    fn distance_from_origin(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// For Point to take 2 different generics
impl<X1, Y1> Point<X1, Y1>
{
    // Mixes two point's values
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>
    {
        Point
        {
            x: self.x,
            y: other.y,
        }
    }
}

// To support different types for each cord
struct Point2<T, U>
{
    x: T,
    y: U,
}

//
enum Option<T>
{
    Some(T),
    None,
}

//
enum Result<T, E>
{
    Ok(T),
    Err(E),
}

fn main() 
{
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    let example = Point {x: 5, y: 4.0}; // Doesnt work. Both must have same type
    let int_flo = Point {x: 5, y: 4.0};

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Some(5);
    let float = Some(5.0);
    // During compilation, performs monomorphization: compiler reads the values that have been used in Option<T> instances
    // and identifies two kinds of Option<T> which is i32 and f64.
    // Expands generic definition of Option<T> into two definitions specialized to i32 and f64
    // Then replace the generic definition with specific ones
    
    // The process is similar to below
    enum Option_i32
    {
        Some(i32),
        None,
    }
    enum Option_f64
    {
        Some(f64),
        None,
    }
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
