use std::fmt;
use std::io::Error;

// Before
pub trait Write
{
    fn write(&mut self, but: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;
    
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

// After 
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn bar() -> !
{
    //
}

fn main() 
{
    // Kilometer is now a synonym for i32
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);  // able to add both values

    // Before
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {}

    // After
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk {} 

    // They dont work. str is a DST, so we dont know how long the string is until runtime
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
    // Must change type to &str (stores the starting position and length of slice)
    // The address of the str and its length

    // To use traits as trait objects, must put them behind a pointer
    // &dyn Trait, Box<dyn Trait>, Rc<dyn Trait>
}

fn generic<T>(t: T)
{
    //
}
// For DST, it is treated as below 
fn generic<T: Sized>(t: T)
{
    //
}

// Default: Generic function works only on types that have a known size at compile time
// But this syntax can get around with it
// ?Size: T may or may not be Sized. It overrides default that generic types must have a known size at compile time
// ?Trait: only available for Sized. Not any other traits
// &T: In case that the type might not be Sized.
fn generic<T: ?Sized>(t: &T)
{

}