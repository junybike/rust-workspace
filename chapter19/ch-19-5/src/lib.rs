// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream
// {
//     // procedural macro takes TokenStream as input and produces one as output
// }

pub trait HelloMacro
{
    fn hello_macro();
}