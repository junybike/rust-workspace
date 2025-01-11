use std::collections::HashMap;

fn main() 
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);    // blue team with 10 points
    scores.insert(String::from("Yellow"), 50);  // yellow team with 50 points

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);   // Accessing score for blue team in scores
    // get method returns an Option<&V>. Returns None if nothing is there.

    // Iterate through HashMap
    for (key, value) in &scores
    {
        println!("{key}: {value}");
    }

    let field_name = String::from("Color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are no more available. Ownership belongs to the 'map'
    // Using reference instead won't work. Values won't go into the HashMap
    // values that the references point to must be valid for as long as the hash map is valid

    // Overwriting a value. Blue score is 25 at the end.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Add a key vand value if a key isnt present
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // or_insert on entry returns a mutable reference to value for corresponding entry key if that key exists.
    // If not, inserts parameter as new value for this key and returns a mutable reference to the new value

    // Finds the frequency of words in sentences
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespaces()    // words separated by white spaces
    {
        // insert new word as a key if this key is new and return the new key. get the key if it exists
        // increment the value that the key holds
        let count = map.entry(word).or_insert(0);   // returns a mutable reference &mut V   
        *count += 1;                                // needs to dereference with *
    }
    println!("{map:?}");
}
