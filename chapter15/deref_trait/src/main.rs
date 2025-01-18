use std::ops::Deref;

struct MyBox<T>(T); // Tuple struct with one element of type T

impl<T> Deref for MyBox<T>
{
    type Target = T;    // defines associated type for Deref trait to use
    
    fn deref(&self) -> &Self::Target
    {
        &self.0 // to return a reference to value we want to access with * operator
    }
    // takes one parameter of type T and returns MyBox instance that holds the value we passed in
    fn new(x: T) -> MyBox<T>    
    {
        MyBox(x)
    }
}

fn hello(name: &str)
{
    println!("Hello {name}");
}

fn main() 
{
    let x = 5;  // holds i32 value
    let y = &x; // holds reference to x

    assert_eq!(5, x);
    assert_eq!(5, *y);  // dereferencing (follow the reference to the value its pointing to)

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // works due to deref coercion
    // since MyBox has deref trait, it can turn &MyBox<String> to &String

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);   // If it didnt have deref coercion
    // (*m) dereferences MyBox<String> to String. & and [..] take string slice of the String
}
