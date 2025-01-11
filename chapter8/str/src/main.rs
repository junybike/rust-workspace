fn main() 
{
    let data = "initial contents";
    let s = data.to_string();       // Converts string literal to String
    let s = String::from("initial contents");   // Creates String from a string literal

    // to_string and String::from does same thing on the example above

    let mut s = String::from("foo");
    s.push_str("bar");                  // Adds "bar" to 's'
    println!("{s}");
    // takes a string slice. no need to take ownership of parameter
    // If push_str took ownership of s, then it wouldnt print

    let mut s = String::from("lo");
    s.push('l');                        // Adds "l" to 's'
    
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;              // s1 is moved here. no longer available. s2 is still available
    // s3 contains Hello World. the add method '+' takes reference. therefore, must use it
    // compiler can coerce the &String to &str. Turns &s2 into &s2[..]

    // Adding multiple at the same time
    let s1 = String::from("A");
    let s2 = String::from("B");
    let s3 = String::from("C");

    let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");  // println! but writes onto 's'.

    // CAUSES ERROR!: doesnt support indexing since it encodes in UTF-8
    let s1 = String::from("hello");
    let h = s1[0];
    // Indexing operations are expected to always take O(1) but not possible with String holding special characters

    // CAUSES ERROR!
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // To iterate over Strings
    for c in "Зд".chars()
    {
        println!("{c}");
    }
    for b in "Зд".bytes()   // print bytes that makes up this string
    {
        println!("{b}");
    }
}
