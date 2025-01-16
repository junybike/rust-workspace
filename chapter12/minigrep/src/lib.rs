use std::error::Error;
use std::fs;
use std::env;

pub struct Config
{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config 
{
    // Constructor for Config 
    pub fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3
        {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // If IGNORE_CASE isnt set to anything, is_ok returns false
        // and perform a case-sensitive search
        let ignore_case = env::var("IGNORE_CASE").is_ok();  
        
        // By passing value in ignore_case variable to Config, it decides if it should search with case sensitive.
        Ok(Config {query, file_path, ignore_case})
    }
}

// Box<dyn Error>: function returns a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config.file_path)?;
    
    // If ignore_case is true, search with case insensitiveness.
    let results = if config.ignore_case
    {
        search_case_insensitive(&config.query, &contents)
    }
    else
    {
        search(&config.query, &contents)
    };

    for line in results
    {
        println!("{line}");
    }
    Ok(())
}

// Lifetime to specify which argument lifetime is connected to the lifetime of the return value
// returned vector should contain string slices that reference slices of argument contents
// data returned by search will live as long as data passed into the search function in contents argument
// * data reference by a slice needs to be valid for reference to be valid 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }
    results
}

// Searching for query in contents with case insensitiveness
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn one_result()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}