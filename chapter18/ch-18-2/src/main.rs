fn main() 
{
    // Causes ERRROR: Attempt to use refutable pattern with let
    let Some(x) = some_option_value;
    
    if let Some(x) = some_option_value
    {
        prinln!("{x}");
    }

    if let x = 5
    {
        println!("{x}");
    }
    // match arms must use refutable patterns except for the last arm
    // as it match any remaining values with an irrefutable pattern
}
