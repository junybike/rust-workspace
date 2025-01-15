pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle
{
    width: u32,
    height: u32,
}
impl Rectangle
{
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: usize) -> usize
{
    a + 2
}

pub fn greeting(name: &str) -> String
{
    format!("Hello {name}");
}

pub struct Guess
{
    value: i32,
}
impl Guess
{
    pub fn new(value: i32) -> Guess
    {
        if value < 1
        {
            panic!("Guess must be greater than or equal to 1. got {value}");
        }
        else if value > 100
        {
            panic!("Guess must be less than or equal to 100. got {value}");
        }
        Guess {value}
    }
}

#[cfg(test)]    // indicates that this is a test function
mod tests {
    use super::*;   // since test module is an inner module, 
                    // need to bring the code under test in outer module into scope of inner module.

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test] // this test always fails
    fn another()
    {
        panic!("Fail test");
    }

    #[test] // Check if a larger rectangle can hold a smaller rectangle
    fn larger_can_hold_smaller()
    {
        let larger = Rectangle
        {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle
        {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test] // Check smaller rectangle cannot hold larger one
    fn smaller_cannot_hold_larger() 
    {
        let larger = Rectangle 
        {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle 
        {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test] // checks the function 'add_two'
    fn it_adds_two()
    {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test] // Custom message when test case failed
    fn greeting_contains_name()
    {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting has no name. value was: {result}");
    }

    #[test]  // This test passes if it throws panic!
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100()
    {
        Guess::new(200);
    }

    #[test] // Returns an Err instead of panicking
    fn it_works() -> Result<(), String>
    {
        let result = add(2, 2);
        if result == 4
        {
            Ok(())
        }
        else
        {
            Err(String::from("2 + 2 != 4"))
        }
    }
    // This enables me to use question mark operator in body of tests
    // This cannot use #[should_panic]. Use assert!(value.is_err()) instead.
}
