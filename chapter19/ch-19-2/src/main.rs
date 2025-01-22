use std::ops::Add;
use std::fmt;

pub trait Iterator
{
    // Item is the placeholder
    type Item;
    fn next(&mut self) -> Option<Self::Item>;   // will return values of type Option<Self::Item>
    // Implementor of Iterator trait will specify concrete type for Item.
    // next method will return an Option containing a value of that concrete type 
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point 
{
    x: i32,
    y: i32,
}
impl Add for Point
{
    type Output = Point;
    fn add(self, other: Point) -> Point
    {
        Point
        {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
//----------------------------------
trait Pilot
{
    fn fly(&self);
}
trait Wizard
{
    fn fly(&self);
}
struct Human;

impl Pilot for Human
{
    fn fly(&self)
    {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human
{
    fn fly(&self)
    {
        println!("Up!");
    }
}
impl Human 
{
    fn fly(&self)
    {
        println!("Waving arms");
    }
}
//------------------------------
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Associated functions that are not methods dont have a self parameter
// rust cant know which type we mean when having multiple type/trait defining non-method functions with same function name
// must use fully qualified syntax

//-----------------------------------
// By specifying that OutlinePrint requires Display trait, 
// we can use to_string function (automatically implemented for any type that implements display)
// without fmt::Display, cannot use to_string
trait OutlinePrint: fmt::Display
{
    fn outline_print(&self)
    {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl fmt::Display for Point // impl display for Point. Can call outline print on a Point
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}. {})", self.x, self.y)
    }
}

struct Point {
    x: i32,
    y: i32,
}
// Error since Point does not implement Display. To fix, must include impl Display for Point
impl OutlinePrint for Point {}

// Creates wrapper type around vec<String> to implement Display
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // self.0 to access inner Vec<T>, since Wrapper is a tuple struct and Vec<T> is the item at index 0 in tuple
        write!(f, "{}", self.0.join(", "))
    }
}

fn main() {
    assert_eq!
    (
        Point {x: 1, y: 0} + Point {x: 2, y: 3},
        Point {x: 3, y: 3}
    );

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // Wants Animal trait with associated non method function baby_name
    // The Animal trait is implemented for struct Dog, 
    // on which also provides associated non method function baby_name directly
    println!("Baby dog is called a {}", Dog::baby_name());
    // Not the desired output. prints spot instead of puppy

    // ERROR. Rust doesnt know which implementation to use. 
    // Animal::baby_name doesnt have a self parameter
    println!("A baby dog is called a {}", Animal::baby_name()); 

    // Fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Associated functions that arent mtehods:
    // there is no receiver. Only list of other arguments.

    let w = Wrapper(vec![String::from("hello"), String::from("World")]);
    println!("w = {w}");
}
