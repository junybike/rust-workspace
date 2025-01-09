enum IpAddrKind
{
    V4,
    V6,
}

enum IpAddr
{
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

// let home = IpAddr::V4(String::from("127.0.0.1"));
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// Struct version of IpAddr

// struct IpAddr
// {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr
// {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr
// {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// }

enum Message
{
    Quit,                       // No data associated with it at all
    Move {x: i32, y: i32},      // named field like struct
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes i32 values
}

// Struct version of Message

// struct QuitMessage;                          // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);                 // tuple struct
// struct ChangeColorMessage(i32, i32, i32);    // tuple struct

impl Message
{
    fn call(&self)
    {
        // implementation
    }
}
let m = Message::Write(String::from("hello"));
m.call();

enum Option<T>
{
    None,
    Some(T),
}


fn main() 
{
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let some_number = Some(5);
    let some_char = Some('e');

    // Similar to Null
    // Must include Option as compiler cannot infer the type that the corresponding Some variant will hold by looking only at None
    let absent_number: Option<i32> = None;  

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    // CAUSES ERROR!
    // Option<i8> and i8 are different types
    // i8: compiler ensure that we always have a valid value
    // Option<i8>: have to worry about possibly not having a value
    // Must convert Option<T> to a T before performing T operations with it
    let sum = x + y; 
}

fn route(ip_kind: IpAddrKind)
{
    // Implementation
}