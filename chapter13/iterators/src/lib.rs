#[test]
fn iterator_demonstration()
{
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // calling the next method on iterator
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // v1_iter must be mutable. Calling next method on iterator 
    // changes internal state that iterator uses to keep track of where it is in the sequence

    // Each call to next eats up an item from iterator
    // In for loop, no need for v1 to be mutable. For loop took ownership of v1_iter.
    
    // values we're getting from calls to next are immutable references to values in vector
    // To take ownership of v1 and returns owned values, use 'into_iter' instead of 'iter'
}

#[test] // consuming adapters
fn iterator_sum()
{
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
// Cannot use v1_iter after call to sum. sum takes ownership of iterator.

#[derive(PartialEq, Debug)]
struct Shoe
{
    size: u32,
    style: String,
}

// filter: takes a closure and it gets an item from iterator and returns a bool
// If true, value will included in iteration produced by filter
// If false, value isnt included.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>
{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
// takes ownership of a vector of shoes and shoe size as parameters. 
// Returns a vector containing only shoese of specified size

// into_iter creates an iterator that takes ownership of vector.
// Then call filter to adapt that iterator into new iterator that only contains elements for which the closure returns true.

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn filters_by_size() 
    {
        let shoes = vec!
        [
            Shoe 
            {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe
            {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe 
            {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec!
            [
                Shoe 
                {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe 
                {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

