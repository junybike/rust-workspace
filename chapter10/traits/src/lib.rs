pub trait Summary   // Consists of the behavior provided by a summarize method
{
    // fn summarize(&self) -> String;  // describes behaviour of the types that implement this trait
    // Compiler enforces that any type that has Summary trait will have method "summarize"
    
    fn summarize_author(&self) -> String;

    // Default implementation of the summarize method
    fn summarize(&self) -> String
    {
        String::from("(Read more from {}...)", self.summarize_author())  
    }

    
}

pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle
{
    fn summarize(&self) -> String
    {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet
{
    fn summarize_author(&self) -> String
    {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String
    {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits as parameters
pub fn notify(item: &impl Summary)  // can call any methods on item that come from the Summary trait
{
    // Pass any instance of NewsArticle or Tweet
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (equivalent to above)
pub fn notify<T: Summary>(item: &T)
{
    println!("Breaking news! {}", item.summarize());
}
// When we want item1 and item2 to have differnet types (both types have Summary)
pub fn notify2(item1: &impl Summary, item2: &impl Summary)
// When we want both to have same types
pub fn notify3<T: Summary>(item1: &T, item2: &T)

// To specify more than one trait bound
// item must implement both Display and Summary
pub fn notify4(item: &(impl Summary + Display))
pub fn notify5<T: Sumary + Display>(item: &T)

// These two are equivalent
fn function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// Returning impl Trait
fn return_summarizable() -> impl Summary
{
    Tweet
    {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
// Returning only the trait it implement is useful in closures and iterators
// can use impl Trait if returning a single type
// Code returning either a NewsArticle or Tweet with the return type impl Summary wouldnt work.

// Use trait bounds to conditionally implement methods
// Pair<T> only implements the cmp_display method if its inner type T implements PartialOrd trait
// that enables comparison and Display trait that enables printing
use std::fmt::Display;

struct Pair<T>
{
    x: T,
    y: T,
}
impl<T> Pair<T>
{
    fn new(x: T, y: T) -> Self
    {
        Self {x, y}
    }
}
impl<T: Display + PartialOrd> Pair<T>
{
    fn cmp_display(&self)
    {
        if self.x >= self.y 
        {
            println!("The largest member is x = {}", self.x);
        } 
        else 
        {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Can conditionally implement a trait for any type that implement another trait
// Blanket implementation: implementation of a trait on any type that satisfies the trait bounds
// ex: std lib implments ToString trait on any type that implements Display trait
impl<T: Display> ToString for T 
{

}
// with the blanket implementation, 
// able to call to_string method defined by ToString trait on any type
// that implements Display trait
let s = 3.to_string(); 