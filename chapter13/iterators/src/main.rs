
// Iterator implementation in std lib
// Requires to define an Item type and the Item type is used in return type of next method
// Item type will be type returned from iterator

// Only requires the next method. Returns one item of iterator at a time wrapped in Some. 
// When iteration is over, returns none
pub trait Iterator
{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() 
{
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();    // holds an iterator for v1.
    
    for val in v1_iter
    {
        println!("Value: {val}");
    }

    // map is an iterator adapter method.
    // takes closure to call on each item as items are iterated through
    // map returns new iterator that produces modifited items
    let v1 = Vec<i32> = vec![1, 2, 3];
    let v2 = Vec<_> = v1.iter().map(|x| x + 1).collect();   // will hold v1 but all values incremented by 1
    assert_eq!(v2, vec![2, 3, 4]);
    // collects results of iterating over iterator thats returned from the call to map into a vector

    // since map takes a closure, we can specify any operation we want to perform on each item.


}
