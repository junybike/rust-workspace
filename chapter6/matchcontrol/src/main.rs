#[derive(Debug)]

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum State
{
    Alabama,
    Alaska,
    // ...
}

fn main() 
{
    let five = Some(5);
    let six = plus_one(five);   
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll
    {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),    // This catch all pattern meets requirement that match must be exhaustive
    }

    // Rule: reroll if rolled a number that is not 3 or 7
    match dice_roll
    {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),    // This catch all pattern meets requirement that match must be exhaustive
    }

    // Rule: nothing else happens if rolled a number that is not 3 or 7
    match dice_roll
    {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),    // This catch all pattern meets requirement that match must be exhaustive
    }

}

fn value_in_cents(coin: Coin) -> u8
{
    // four arms
    match coin
    {
        Coin::Penny =>
        {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin:Dime => 10,
        Coin::Quarter => 
        {
            println!("State quarter from {state:?}");
            25
        }

    }
}

fn lucky_penny(coin: Coint) -> u8
{
    match coin
    {
        
        Coin::Nickel => 5,
        Coin:Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
    match x
    {
        None => None,           // None case must exists to compile (prevents from assuming that we have a value when we might have null)
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}