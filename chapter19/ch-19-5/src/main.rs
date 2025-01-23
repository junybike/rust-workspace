// 
// #[macro_export]      -> indicates that this macro is  available whenever crate in which the macro is defined is brought into scope
// macro_rules! vec {   -> the name of the macro
//     ( $( $x:expr ),* ) => {  -> ($) to declare a variable in macro system.
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);   
//             )*
//             temp_vec
//         }
//     };
// }
// ( $( $x:expr ),* ) => {  
// ($) to declare a variable in macro system.
// comma separator: indicates that literal comma separator character can appear after the  code matches the code in $()
// * specifies that pattern matches zero or more of whatever precedes the *. 

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

// Attribute like macros
// route that annotates functions when using web app framework
// #[route]: defined by framework as procedural macro
#[route(GET, "/")]
fn index() {}

// first TokenStream: contents of the attribute: the GET, "/" part
// second: body of item the attribute is attached to: fn index() {} and rest of functions body
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

fn main() 
{
    // let v: Vec<u32> = vec![1, 2, 3];
    // when the macro is called, $x pattern matches the three expression: 1, 2, 3
    Pancakes::hello_macro();

    // Example of function like macro: sql!
    let sql = sql!(SELECT * FROM posts WHERE id=1);
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}