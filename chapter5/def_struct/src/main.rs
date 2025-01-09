struct User
{
    active: bool,
    username: String,       // setting this type as &str will cause an error
    email: String,          // requires the use of lifetimes
    sign_in_count: u64,
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit like struct (without any fields)
struct AlwaysEqual;

fn main() 
{
    let user1 = User
    {
        active: true,
        username: String::from("me123"),
        email: String::from("me@gmail.com"),
        sign_in_count: 1,
    };

    // Entire instance must be mutable 

    user1.email = String::from(("me123@gmail.com"));

    let user2 = User
    {
        email: String::from("you@gmail.com");
        ..user1 // let other fields be the same as user1's field
    };

    // user1 is no longer available since its username is moved to user2.
    // if only active and sign_in_count were moved, user1 wouldve still available.

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Each struct we define is its own type. Even if the fields have the same types.
    // May destructure them into individual pieces and use . followed by index to access individual values

    let subject = AlwaysEqual;

}
 
fn build_user(email: String, username: String) -> User
{
    User 
    {   // Using Field init shorthand, no need to explictly be "username: username"
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}