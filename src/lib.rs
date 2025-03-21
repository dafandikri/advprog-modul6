use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

/// Custom error type for ThreadPool creation
#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
    ThreadCreationError(String),
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender) 
        }
    }

    /// Build a new ThreadPool with error handling.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// Returns an error if the size is zero or if thread creation fails.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroSize);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            match Worker::build(id, Arc::clone(&receiver)) {
                Ok(worker) => workers.push(worker),
                Err(err) => return Err(PoolCreationError::ThreadCreationError(
                    format!("Failed to create worker {}: {}", id, err)
                )),
            }
        }

        Ok(ThreadPool { 
            workers, 
            sender: Some(sender) 
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        // Send terminate message to all workers
        for _ in &self.workers {
            self.sender.as_ref().unwrap().send(Message::Terminate).unwrap();
        }
        
        // Take the sender option to ensure it's dropped
        self.sender.take();
        
        println!("Shutting down all workers");
        
        // Join all worker threads
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Message::Terminate => {
                    println!("Worker {id} was told to terminate.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }

    fn build(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Result<Worker, std::io::Error> {
        let thread = match std::thread::Builder::new()
            .name(format!("worker-{}", id))
            .spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {id} was told to terminate.");
                        break;
                    }
                }
            }) {
                Ok(thread) => thread,
                Err(e) => return Err(e),
            };

        Ok(Worker { id, thread: Some(thread) })
    }
}
