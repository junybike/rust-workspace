use std::env;
use std::process;

use minigrep::Config;

fn main() 
{
    // Collecting command line args into a vector.
    // unwrap_or_else: custom non-panic! error handling
    // by passing the inner value of err (the string: not enough arguments)
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // dbg!(args); prints value inside args

    // println!("Searching for: {}", config.query);
    // println!("In file: {}", config.file_path);

    // Opens file and returns value of type std::io::Result<String> 
    // also checks whether run returns Err value. If so, calls process::exit(1) 
    if let Err(e) = minigrep::run(config)
    {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

