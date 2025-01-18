enum List
{
    Cons(i32, Box<List>),   // Cons variant needs size of i32 + space to store box's pointer data
    Nil,                    // Since we now know that each List will take upto size of i32 + box's pointer data
                            // it is now no longer infinite 
}

use crate::List::{Cons, Nil};

fn main() {
    // points to the value 5 which is allocated on heap
    // also gets deallocated when the box goes out of scope
    let b = Box::new(5);    
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
