fn main() 
{
    let s = "hello";

    {                       // s is not valid yet.
        let s = "hello";    // s is valid from this point
    }                       // scope is over. s is no longer valid

    let s = String::from("Hello");      // Creates a String from a string literal
    let mut s = String::from("Hello");  // mutable string
    s.push_str(", world!");             // appends a literal to a String
    println!("{s}");

    let x = 5;  // stack: bind value 5 to x 
    let y = x;  // stack: bind value x to y

    {
        let s1 = String::from("Hello");
        // String data is copied. (the pointer, length, capacity that are on the stack)
        // Does not copy data on the heap that the pointer refers to

        // -- move occurs because s1 has type String which doesnt implement the 'copy' trait

        let s2 = s1;               
        // -- value moved here

        // Rust considers s1 as no longer valid to ensure memory safety
        // Therefore, no need to free anything when s1 goes out of scope

        // CAUSES ERROR! Attempt to use invalidated reference
        // println!("{s1}");     
    
    }   // Will only free s2   
    
    {
        let mut s = String::from("Hello");
        s = String::from("Ahoy");           // Drops original value's memory and uses this memory
        println!("{s}, world!");            // Prints "Ahoy, world!"
    }

    {
        let s1 = String::from("HEllo");
        let s2 = s1.clone();                // Deep copy the heap data of the String
        println!("s1 = {s1], s2 = {s2}");
    }

    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s moves into the function.
                                    // s is no longer valid from here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x moves into the function    
                                    // i32 has copy trait. Still valid from here

    let s1 = gives_ownership();         // the function moves its return value into s1 
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved to the function. the function return value to s3

    // Example
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("Length of {s2} is {len}");
}

fn takes_ownership(str: String)
{
    // str comes into this scope
    println!("{str}");
    // str goes out of scope and drop is called. memory is freed
}

fn makes_copy(i: i32)
{
    // i comes into this scope
    println!("{i}");
    // i goes out of scope
}

fn gives_ownership() ->String 
{
    let str = String::from("yours");    // str comes into scope
    
    // str is returned and moves out to the calling function
    str
}

fn takes_and_gives_back(str: String) -> String
{
    // str is returned and moves out to the calling function
    str
}

fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len();
    (s, length);
}