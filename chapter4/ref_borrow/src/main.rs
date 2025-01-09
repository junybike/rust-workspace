fn main() 
{
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);        // allow function to refer to value of s1 without owning it
    println!("Length of {s1} is {len}");

    let s = String::from("hello");

    // CAUSES ERROR!: Tries to modify a referenced value
    // change(&s);

    let mut s = String::from("hello");
    m_change(&mut s);
    println!("{s}");

    // CAUSES ERROR!: Cannot borrow s as mutable more than once at a time
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    // Cannot have immutable and mutable reference on the same value at the same time
    let mut s = String::from("hello");
    let r1 = &s;        // ok
    let r2 = &s;        // ok
    let r3 = &mut s;    // not ok
    println!("{}, {}, {}", r1, r2, r3);

    // This is ok
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");
    let r3 = &mut s;
    println!("{r3}");

    // CAUSES ERROR!
    // Dangling pointer
    let ref_nothing = dangle();
}

fn calculate_length(s: &String) -> usize    // s is a reference to a String
{
    s.len() // s isnt dropped as the scope does not have ownership of what it refers to
}

// fn change(str: &String)
// {
//     str.push_str(", world");
// }

fn m_change(str: &mut String)
{
    str.push_str(", world!");
}


fn dangle() -> &String
{
    let s = String::from("hello");  // s is a new String
    &s                              // returns a reference to String s.
}   // s goes out of scope and dropped... Memory goes away
