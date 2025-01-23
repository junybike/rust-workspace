use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream
{
    // Constructs a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_hello_macro(&ast)
}

// hello_macro_derive is responsible for parsing the TokenStream

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream
{
    let name = &ast.ident;  // contains the name (identifier) of annotated type
    let gen = quote!        // lets us define the Rust code that we want to return
    {
        impl HelloMacro for #name
        {
            fn hello_macro()
            {
                println!("Hello macro, my name is {}", stringify!(#name));
                // stringify: takes rust expression and turns it into string literal
                // 1 + 2 -> "1 + 2"
            }
        }
    };
    gen.into()  // takes intermediate representation and returns TokenStream type
}