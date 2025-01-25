
use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};


pub struct Server {
    busy_threads: u32,
    max_threads: u32,
    task_queue: VecDeque<Box<dyn HeliumTask + Send>>
}

impl Server {

    pub fn new(max_threads: u32) -> Self {
        todo!();
        Server {
            busy_threads: 0,
            max_threads,
            task_queue: VecDeque::new()
        }
    }


    fn create_thread_pool(thread_cap: u32, task_queue: Arc<Mutex<VecDeque<Box<dyn HeliumTask + Send>>>>) -> Vec<thread::JoinHandle<()>> {
    
        let mut thread_pool = Vec::new();

        for _ in 0..thread_cap {
    
            let task_mutex = Arc::clone(&task_queue);

            thread_pool.push(thread::spawn(move || {
                /* Thread task handler */
                loop {
                    let lock_res = task_mutex.lock();
                    if lock_res.is_err() {
                        return ();
                    }
                    let mut tasks = lock_res.unwrap();
                    
                    // Handle the first task in the queue
                    let task = match tasks.pop_front() {
                        None => continue,
                        Some(t) => t
                    };
                    
                    let task_result = task.execute();
                    
                    if task_result.is_err() {
                        // Idk, log error or somethin
                    }
                }
            }));
        }
        
        thread_pool
    }

    pub fn route(&mut self) {
        todo!();
    }


    pub fn run(&mut self) -> std::io::Result<()> {

        todo!();
    }
}


/// A HeliumTask represents the behaviour of tasks (handlers) for a Helium Server.
pub trait HeliumTask {
    fn execute(&self) -> std::io::Result<()>;
}



