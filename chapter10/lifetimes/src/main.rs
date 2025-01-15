// This function does not compile
// Don't know the reference being returned refers to x or y.
fn longest(x: &str, y: &str) -> &str
{
    if x.len() > y.len()
    {
        x
    }
    else
    {
        y
    }
}

// specifies that all references in signature must have same lifetime 'a
// lifettime of reference returned by this function is 
// same as the smaller of the lifetimes of the value referred to by function arguments
fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str
{
    if x.len() > y.len()
    {
        x
    }
    else
    {
        y
    }
}
// specifying lifetime parameter does not change lifetime of values passed in or returned.
// Specifies that borrow checker should reject any values that don't adhere to these constraints

// If always returns first parameter, no need to specify lifetime on y parameter
// y is not related to lifetime of x or the return value
fn longest3<'a>(x: &'a str, y: &str) -> &'a str
{
    x
}

// When returning a reference from a function, lifetime parameter for return type needs to match the liftime parameter for one of the parameters.
// If reference returned doesnt refer to one of parameters, it must refer to value created within this function
// But this cause dangling reference as value goes out of scope at the end of function
fn longest4<'a>(x: &str, y: &str) -> &'a str
{
    let result = String::from("really long string");
    result.as_str() 
    // return value lifetime is not related to lifetime of parameters
    // result gets cleaned up at the end of function
}

// struct that holds reference. Requires lifetime annotation
struct ImportantExcerpt<'a>
{
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a>
{
    fn level(&self) -> i32 
    {
        3
    }

    // apply first lifetime elision rule. (gives both their own lifetime)
    // apply third lifetime (return type gets lifetime of &self)
    fn announce_and_return_part(&self, announcement: &str) -> &str
    {
        println!("{announcement}");
        self.part
    }
}

// Syntax of specifying generic type parameters, trait bounds, and lifetimes
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>
(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len()
    {
        x
    }
    else
    {
        y
    }
}

fn main() 
{
    let r;
    {
        let x = 5;
        r = &x;
    }
    // r has dangling pointer!

    // This part works.
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest2(string1.as_str(), string2.as_str());
        println!("The longest string: {result}");
    }

    // This part causes error.
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");

    // Holds a reference to first sentence of the String owned by 'novel'
    // 'novel' doesnt go out of scope until after the ImportantExcerpt goes out of scope.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt 
    {
        part: first_sentence,
    };

    // Static lifetime
    // Affected reference can live for the entire duration of the program
    let s: &'static str = "static liftime";
}

fn first_word(s: &str) -> &str {}
// apply first rule (each parameter gets its own lifetime)
fn first_word<'a>(s: &'a str) -> &str {}
// apply second rule (because exactly one input lifetime) (lifetime of one input parameter gets assigned to output lifetime)
fn first_word<'a>(s: &'a str) -> &'a str {} 

fn longest(x: &str, y: &str) -> &str {}
// apply first rule
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// no second rule application
