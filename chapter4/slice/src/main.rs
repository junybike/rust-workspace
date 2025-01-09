fn main() 
{
    let mut s = String::from("hello world");
    let word = first_word2(&s);

    // CAUSES ERROR!
    // clear truncates the String. must get mutable reference
    // println uses reference in word so immutable reference must be active.
    // therefore mutable and immutable refernce at the same time. 
    s.clear();
    println!("{word}");
    

    let s = String::from("Hello world");
    let s1 = &s[0..5];      // [..5]
    let s2 = &s[6..11];     // [6..]
    let s3 = &s[0..11];     // [..]        

    let s = "Hello world";  // the type is &str. this types are immutable reference. a slice pointing to that specific point of the binary 

    let s = String::from("hello world");
    let word = first_word2(&s[0..6]);       // works on slices of String

    let strliteral = "hello world";
    let word = first_word2(&strliteral);    // works since string literals are string slices already.

    // array example
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize
{
    // To check String in element by element and check whether a value is a space,
    // converts String to an array of bytes
    let bytes = s.as_bytes();    

    // Iterator over the array of bytes
    // iter: returns each element in a collection
    // enumerate: wraps the result of iter and returns eacah element as part of a tuple
    // First element of the tuple returned from enumerate is the index and second element is a reference to the element
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' ' // If the byte represents the space, return its position.
        {
            return i;
        }
    }

    s.len() // If no space is found, the whole thing is a word. return its length.
}

fn first_word2(s: &String) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }
    &s[..]
}