fn main() 
{
    let v: Vec<i32> = Vec::new();   // Able to hold multiple i32 values
    let v = vec![1, 2, 3];          // A vec macro. Holds i32 values (default)

    let mut v = Vec::new();
    v.push(5);                  // modifies 'v' by pushing 5 into it

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    printlnt!("Third element: {third}");

    let third: Option<&i32> = v.get(2);
    match third
    {
        Some(third) => println!("Third element: {third}"),
        None => println!("Theres no third element"),
    }

    let a = &v[100];        // CAUSES ERROR!
    let b = v.get(100);     // Gets "None" or Some(&element)

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);          // CAUSES ERROR!
    // Cannot add an element to a vector while holding a reference to an item
    // vectors put values next to each other in memory. 
    // adding new element to the end of vector may require allocating new memory and copying old elements to new space.
    // then, the reference to the first element would point to deallocated memory.

    // Iterating over values in vector
    let v = vec![100, 32, 57];
    for i in &v
    {
        println!("{i}");
    }

    // Changing values in vec
    let mut v = vec![100, 32, 57];
    for i in &mut v
    {
        *i += 50;
    }

    // To store multiple types in a vector
    enum SpreadSheetCall
    {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec!
    [
        SpreadSheetCall::Int(3),
        SpreadSheetCall::Text(String::from("blue")),
        SpreadSheetCall::Float(10, 12),
    ];
    // Need to use match expression to handle every possible case
    // to elimnate chance that one or more types would cause errors with operations performed on elements of the vector
    
    // vectors and all its values get freed once it goes out of scope.

}
