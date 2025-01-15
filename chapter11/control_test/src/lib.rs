pub fn add_two(a: usize) -> usize
{
    a + 2
}

#[cfg(test)]
mod tests
{
    use super::*;

    // Do: cargo test add
    // to run all test cases that contain 'add' in their names
    #[test]
    fn add_two_and_two()
    {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two()
    {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    // Do: cargo test one_hundred
    // to run just this test
    #[test] 
    fn one_hundred()
    {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    fn it_works() 
    {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // To run this test, do: cargo test -- --ignored
    #[test]
    #[ignore]   // Indicate that we want to exclude this test case
    fn expensive_test() 
    {
        // code that takes an hour to run
    }
}