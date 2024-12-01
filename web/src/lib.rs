use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

/// mpsc - multiple producer, single consumer
/// a channel for sending data from multiple threads to one receiver
/// sender -> receiver
///        -> receiver

/// arc - atomic reference counting - smart pointer that lets multiple threads share the same data safely

/// Mutex - mutual exclusion - ensures only one thread can access a shared resource

/// Simplify the type signature for the jobs
///
/// `dyn FnOnce()`  - A trait for a closure that only runs once
/// `Send`          - Ensures the job can be safely sent to another thread
/// `'static`       - Ensures the job does not reference data that might go out of scope
type Job = Box<dyn FnOnce() + Send + 'static>;

// Worker struct, represents a single thread in the thread pool
struct Worker {
    id: usize, // Unique identifier for the worker
    thread: thread::JoinHandle<()>, // The thread associated with the worker.
}

impl Worker {
    /// Create a new Worker
    ///
    /// # Arguments
    /// * `id` - The ID of the worker
    /// * `receiver` - Shared, thread-save receiver for getting jobs
    ///
    /// The worker threads runs in an infinite loop, fetching and executing jobs from the receiver.
    fn new(id:usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        // Start a new thread and pass it to a closure
        // `move` captures variables b value to transfer them into the new thread
        let thread = thread::spawn(move || loop {
            // Lock the receiver to safely access the next job
            let job = receiver
                .lock() // Lock the resource
                .unwrap()// Handle potential errors
                .recv()// (receive) Wait for and receive the next job from the channel
                .unwrap();

            println!("Worker {id} got a job; executing.");

            // Execute the job
            job();
        });

        Worker { id, thread } // Return the Worker instance
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// # Arguments
    /// * `size` - The number of workers (threads) in the pool
    ///
    /// # Panics
    ///
    /// Ensures at least one worker exists and workers and channels.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // Panics if less than 1

        // Initialize a vector with capacity to hold the specified number of workers
        let mut workers = Vec::with_capacity(size);

        // Create a channel for sending jobs and receiving them in worker threads
        let (sender, receiver) = mpsc::channel();

        /// Wrap the receiver in Mutex and Arc
        /// - Mutex for thread-safe access
        /// - Arc for shared ownership
        let receiver = Arc::new(Mutex::new(receiver));

        // Create the specified amount of workers
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))) // Create a new reference to the receiver for each worker
        }

        ThreadPool { workers, sender } // Return the ThreadPool instance
    }

    /// Execute a job by sending it to the thread pool
    ///
    /// # Arguments
    /// * `f` - A closure of function to execute in a worker thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // Ensure the closure meets the necessary constraints
    {
        // Wrap the job in a Box to make it heap-allocated so it can be sent through the channel
        let job = Box::new(f);

        // Send the job to the workers via the channel
        self.sender.send(job).unwrap();
    }
}