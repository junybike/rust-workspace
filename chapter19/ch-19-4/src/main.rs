fn add_one(x: i32) -> i32
{
    x + 1
}

// f: a function that takes i32 parameter and returns an i32
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32
{
    f(arg) + f(arg)
}

fn main() 
{
    let answer = do_twice(add_one, 5);
    println!("ANSWER: {answer}");

    // closure defined inline or a named function
    // turns a vector of numbers into a vector of strings
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings = Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    //                                = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status 
    {
        Value(u32),
        Stop,
    }
    // Creates Status::Value instances using each u32 value in range that map is called on 
    // by using the initializer function of Status::Value
    let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // Returning closures
    fn returns_closure() -> Box<dyn Fn(i32) -> i32>
    {
        Box::new(|x| x + 1)
    }
}

