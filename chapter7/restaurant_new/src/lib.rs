mod front_of_house 
{
    pub mod hosting 
    {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // hosting is now valid name in this scope
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() 
{
    hosting::add_to_waitlist();
    // add_to_waitlist();
}

mod customer 
{
    pub fn eat_at_restaurant()
    {
        // Error since hosting is not defined in this scope. Fix by including super:: within child customer module
        hosting::add_to_waitlist(); 
    }
}

// as keyword
use std::fmt::Result;
use std::io::Result as IoResult 

// Re-exporting names with pub use
// let any code to use from a new scope
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant()
{
    // This is valid   
    restaurant::hosting::add_to_waitlist(); // restaurant:: plays as pub
}

// Nested paths
// Instead of below
use std::cmp::Ordering;
use std::io;
// Use
use std::{cmp::Ordering, io};
// Instead of below
use std::io;
use std::io::Write;
// Use
use std::io::{self, Write};

// Glob operator: bring all public items defined in path into scope
use std::colections::*;
