#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor
{
    Red,
    Blue,
}

struct Inventory
{
    shirts: Vec<ShirtColor>,
}
impl Inventory
{
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor
    {
        // takes one arrguments: closure without any agrument that returns a value T
        // If Option<T> is Some variant, unwrap returns the value from within the Some.
        // If None variant, it calls closure and erturns value retured by the closure

        // The closure captures immutable reference to self Inventory instance and passes it with code we specify to unwrap method.
        // Which functions cannot do
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor
    {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts
        {
            match color
            {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue
        {
            ShirtColor::Red
        }
        else
        {
            ShirtColor::Blue
        }
    }
}

// std unwrap method implementation
// F must be able to call once, take no arguments and return T.
// If Option is Some, f() wont be called. If None, f() gets called once
impl<T> Option<T> 
{
    pub fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T
    {
        match self
        {
            Some(x) => x,
            None => f(),
        }
    }
}

#[derive(Debug)]
struct Rectangle 
{
    width: u32,
    height: u32,
}

fn main()
{
    let store = Inventory
    {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("User with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("User with preference {:?} gets {:?}", user_pref2, giveaway2)

    // Annotating types for a closure
    // Defines a closure and store in a variable 
    let expensive_closure = |num: u32| -> u32
    {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Comparison
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // CAUSES ERROR: attempt to call closure whose types are inferred with two diff types
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    // Defining and caalling closure that captures an immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Allowed to have multiple immutable references
    let only_borrow = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrow();
    println!("After calling closure: {list:?}");

    // Define and call closure that captures a mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrow_mutably = || list.push(7);
    // println statement between closure declaration and calling is not allowed
    // borrow_mutably captures a mutable reference to list.
    
    // No immutable borrow to print isnt allowed. 
    // No other borrows allowed when there's mutable borrow
    borrow_mutably();
    println!("After calling closure: {list:?}");

    // To force the closure to take ownership of values even if they dont strictly need it,
    // use 'move'. Useful for thread, as the thread owns the ownership.
    // Prevention: In case main thread ends before the new thread.
    let list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");
    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap();

    // Ordering rectangle by width
    // sort_by_key is an FnMut. It calls the closure multiple times (once for each item in the slice)
    // |r| r.width doesnt capture, mutate, or move out anything from its environment. Therefore, meets trait bound requirement
    let mut list = 
    [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    // Example of closure that implements just FnOnce trait
    // Attempts to do counting by pushing value from closure's environment into sort_operations vector.
    // Closure captures value then moves value out of closure by transferring ownership of value to sort_operations vector
    // ^ can be called once. Calling again wont work since value would no longer be in environment to be pushed into sort_operations.
    let sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {sort_operations.push(value); r.width});
    println!("{list:#?}");

    // Changed closure body so that it doesnt move values out of the environment.
    // Works since it only captures mutable reference to num_sort_operations counter. Therefore, can be called more than once. 
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {num_sort_operations += 1; r.width});
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}