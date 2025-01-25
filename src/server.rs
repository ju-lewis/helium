
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


    fn create_thread_pool(thread_cap: u32, task_queue: Arc<Mutex<VecDeque<Box<dyn HeliumTask + Send>>>>) -> Vec<thread::JoinHandle<std::io::Result<()>>> {
    
        let mut thread_pool = Vec::new();

        for _ in 0..thread_cap {
            thread_pool.push(thread::spawn(|| {
                /* Thread task handler */
                
                Ok(())
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



pub trait HeliumTask {
    
}



