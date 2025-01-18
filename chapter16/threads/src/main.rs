use std::thread;
use std::time::Duration;

fn main() 
{
    // The thread and main will display concurrently
    // Thread cannot print upto 10, since main thread shuts down after printing upto 5
    thread::spawn(|| {
        for i in 1..10
        {
            println!("Number {i} from thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();  // Placing the join here waits until thread finishes. Then move on.

    for i in 1..5
    {
        println!("Number {i} from main");
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap(); // Placing the join here forces program to wait for all thread to finish

    // Causes error
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {v:?}");
    // });
    // handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move|| {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}
