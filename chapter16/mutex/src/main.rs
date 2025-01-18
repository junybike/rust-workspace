use std::sync::{Arc, Mutex};
use std::thread;

fn main() 
{
    let m = Mutex::new(5);
    {
        // To access the data in mutex, use lock method to aquire the lock
        // blocks current thread so it cant do any work until its our turn to have the lock

        // lock would fail if another thread holding the lock panicked.
        // unwrap also calls panic if we're in this situation

        // after acquiring lock, we can threat return value 'num' as a mutable reference to data inside
        // type of m is Mutex<i32>. (not i32). Therefore, must call lock to use the i32 value
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {m:?}");

    // multiple ownership with multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10
    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||
        {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlees.push(handle);
    }

    for handle in handles
    {
        handle.join().unwrap();
    }
    println!("result: {}", *counter.lock().unwrap());
}
