use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// Define a trait called `FnBox` that allows calling boxed closures.
trait FnBox {
    fn call_box(self: Box<Self>);
}

// Implement the `FnBox` trait for any type `F` that is a closure taking no arguments and returning nothing.
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// Define a type alias `Job` for a boxed closure taking no arguments, Sendable between threads, and 'static lifetime.
type Job = Box<dyn FnOnce() + Send + 'static>;

// Define the ThreadPool struct.
pub struct ThreadPool {
    workers: Vec<Worker>,              // A vector to hold worker threads.
    sender: Option<mpsc::Sender<Job>>, // An optional channel sender used for submitting jobs to worker threads.
}

// Define the Worker struct.
struct Worker {
    id: usize,                              // An ID to identify each worker.
    thread: Option<thread::JoinHandle<()>>, // An optional handle to the thread executing the worker.
}

impl ThreadPool {
    // Constructor to create a new ThreadPool with a specified size.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // Ensure the size is at least 1.

        // Create a channel to communicate between threads. Sender will be used to submit jobs.
        let (sender, receiver) = mpsc::channel();

        // Wrap the receiver in an Arc and Mutex to share it among worker threads safely.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        // Create the specified number of worker threads and store them in the workers vector.
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Return the ThreadPool instance with the worker vector and sender channel.
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // Function to execute a closure in the thread pool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Wrap the closure in a Box to store it on the heap.
        let job = Box::new(f);

        // Send the job to the worker threads through the channel.
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Worker {
    // Constructor to create a new Worker with a given ID and access to the receiver channel.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Create a new thread that will execute the worker's function.
        let thread = thread::spawn(move || loop {
            // Receive a job from the channel (blocks if no job is available).
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job(); // Execute the received closure.
                }
                Err(_) => {
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

impl Drop for ThreadPool {
    // Implement the Drop trait to clean up the thread pool properly.
    fn drop(&mut self) {
        // Iterate over workers and join the associated threads to wait for their completion.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // If the worker's thread handle is still present, join the thread to wait for its completion.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
