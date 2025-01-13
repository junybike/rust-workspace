use std::fs::File;
use std::io::{self, Read};
use std::fs;

// Reads a username from a file.
// If the file doesnt exist/cant be read, function will return those errors to the code that called the function
fn read_username_from_file() -> Result<String, io::Error>   // Returning Result<T, E> where T gets filled with String and E gets filled with io::Error
{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result
    {
        Ok(file) => file,           // If success, username_file holds the file handle 'file'
        Err(e) => return Err(e),    // If error, returns error value and skips anything below. 
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username)
    {
        Ok(_) => Ok(username),  // Assign username 
        Err(e) => Err(e),       // return error value
    }
}
// Code calling this function will deal with either a username String or Err value that contains an io::Error

// Using ? operator (similar to match )
fn read_username_from_file2() -> Result<String, io::Error>
{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// If value of Result is an Ok, value inside the Ok will get returned from the ? expression and program continues.
// If value is an Err, Err is returned from the whole function like with a return call

// ? operator simpler version
fn read_username_from_file3() -> Result<String, io:Error>
{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error>
{
    fs::read_to_string("hello.txt") // Opens file, creates new string, reads contents of the file, put contents into that String
}

// To use ? in main
fn main() -> Result<(), Box<dyn Error>>
{
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
// Box<dyn Error> is a trait object. Means any kind of error
// Allows main to return any Err values to early 