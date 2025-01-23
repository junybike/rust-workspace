use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

pub struct Worker
{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool
    {
        // Creates new ThreadPool.
        // The size is the number of threads in the pool
        // This function panics if the size is zero
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // Arc will let multiple workers own the receiver
        // Mutex ensures that only one worker gets a job from receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));  
        let mut workers = Vec::with_capacity(size);

        for id in 0..size
        {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers, 
            sender: Some(sender),
        }
    }
    
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,
    {
        // Send the job down the sending end of the channel
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap(); // unwraps in case that sending fails
    }
}
impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        drop(self.sender.take());   // dropping sender closes channel. No more messages will be sent
        for worker in &mut self.workers
        {
            println!("Shutting down worker {}", worker.id);
            
            // take method on Option takes Some variant out and leaves None in its place
            if let Some(thread) = worker.thread.take()
            {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker
{
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = thread::spawn(move || loop{
            // lock: to acquire the mutex
            // unwrap: panic on any error
            // recv: receive the job if lock is acquired
            let message = receiver.lock().unwrap().recv();
            
            match message 
            {
                Ok(job) => 
                {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) =>
                {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {
            id, 
            thread: Some(thread),
        }
    }
}