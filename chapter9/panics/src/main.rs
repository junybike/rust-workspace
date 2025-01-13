use std::fs::File;

fn main() 
{
    // Will cause program to stop and display message below
    panic!("CRASH AND BURN");

    // Will cause panic! due to attempt to read beyond
    let v = vec![1, 2, 3];
    v[99];

    // Return type of File::open is a Result<T, E>
    // T: filld by the implementation of File::open with the type of the success value std::fs::File
    // E: std::io::Error
    // In success case of File::open, file_result will be an instance of Ok that contains a file handle
    // In fail case, file_result will be instance of Err that contains more info about the kind of error that occurred
    let file_result = File::open("hello.txt");

    // Type of value that File::open returns inside the Err variant is io::Error.
    // The struct provided by stdlib. Including method 'kind' that we can call to get an io::ErrorKind value. 

    let text_file = match file_result
    {   
        Ok(file) => file,   // return inner file value out of the Ok variant and assign file handle value to text_file
        
        // Check whether the value returned by error.kind() is NotFound variant of ErrorKind enum.
        // Creates new file if it is.
        Err(error) => match error.kind
        {
            ErrorKind::NotFound => match File::create("hello.txt")
            {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file. {e:?}"),
            },
            other_error => 
            {
                panic!("Problem opening file {other_error:>}");
            }
        },
    };

    // Same functionality as above but without using match
    let text_file = File::open("hello.txt").unwrap_or_else(|error|
    {
        if error.kind() == ErrorKind::NotFound
        {
            File::create("hello.txt").unwrap_or_else(|error|
            {
                panic!("Problem creating the file: {error:?}");
            })
        }
        else
        {
            panic!("Problem opening the file! {error:?}");
        }
    });

    // If the code is ran without hello.txt, it will call the 'unwrap' method
    let text_file = File::open("hello.txt").unwrap();
    // Same as above but displays a message
    let text_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    
}
