fn main() 
{
    let config_max = Some(3u8);
    match config_max
    {
        Some(max) => println!("maximum: {max}"),    // If the value is Some, print out the value in Some
        _ => (),                                    // Does not do anything with None
    }    

    // Use if let
    let config_max = Some(3u8);
    if let Some(max) = config_max       // The pattern: Some(max)
    {
        println!("maximum: {max}");
    }

    let mut count = 0;
    match coin 
    {
        Coin::Quarter(state) => println!("quarter from {state:?}"),
        _=> count += 1,
    }

    // Use if let
    let mut count = 0;
    if let Coin::Quarter(state) = coin
    {
        println!("quarter from {state:?}");
    }
    else 
    {
        count += 1;
    }
}
