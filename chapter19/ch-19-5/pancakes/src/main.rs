use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes
{
    fn hello_macro()
    {
        println!("Hello macro, my name is pancakes");
    }
}

// impl hello macro function is responsible for transforming the syntax tree

fn main() 
{
    // let v: Vec<u32> = vec![1, 2, 3];
    // when the macro is called, $x pattern matches the three expression: 1, 2, 3
    Pancakes::hello_macro();
}
