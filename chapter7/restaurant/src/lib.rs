mod front_of_house 
{
    pub mod hosting 
    {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving 
    {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house
{
    fn fix_incorrect_order()
    {
        cook_order();
        super::deliver_order();     // Goes back to back_of_house module and get to delier_order which is at same relationship
    }
    fn cook_order() {}

    pub struct Breakfast        // only toast is public
    {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer  // All variants are public 
    {
        Soup,
        Salad,
    }

    impl Breakfast 
    {
        pub fn summer(toast: &str) -> Breakfast
        {
            Breakfast
            {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// hosting and serving are siblings defined within front of house

pub fn eat_at_restaurant()
{
    crate::front_of_house::hosting::add_to_waitlist();  // absolute path
    front_of_house::hosting::add_to_waitlist();         // relative path

    let order1 = back_of_house::Appetizer::Soup;

    let mut meal = back_of_house::Breakfast::summer("rye"); // order a breakfast in summer with rye toast
    meal.toast = String::from("wheat");                     // change the bread type. able to access toast since it is public
    println!("I'd like {} toast please," meal.toast);      
}

// if moving front_of_house module and eat_at_restaurant function into another module, need to update absolute path.
// if eat_at_restaurant is moved to another module, need to update relative path

// Absolute path: start with crate -> front_of_house module (not public) is defined in crate root 
// (but eat_at_restaurant is defined in same module as front_of house. They are siblings) can refer front_of_house from eat_at_restaurant 
// hosting module is public -> can access parent module of hosting. Therefore we can access hosting
// Add_to_waitlist is public and able to access its parent module. Therefore, function calls.

// relative path: starts from front_of_house which is defined within same module as eat_at_restaurant. 
// rest are same as absolute path

