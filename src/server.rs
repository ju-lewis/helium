
use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};
use crate::{http::{Path, StatusCode}, Method};

pub struct Server {
    busy_threads: u32,
    max_threads: u32,
    thread_pool: Vec<thread::JoinHandle<()>>,

    //task_queue,
    //handlers: HashMap<Path, HeliumTask>
}

impl Server {

    pub fn new(max_threads: u32) -> Self {

        let task_queue = VecDeque::new();
        
        let thread_pool = Self::create_thread_pool(max_threads, Arc::new(Mutex::new(task_queue)));

        Server {
            busy_threads: 0,
            max_threads,
            thread_pool
        }
    }


    fn create_thread_pool(thread_cap: u32, task_queue: Arc<Mutex<VecDeque<Box<dyn HeliumTask<> + Send>>>>) -> Vec<thread::JoinHandle<()>> {
    
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

                    let task_result: TaskResponse = task.execute();
                    
                    // Return response over TCP stream

                }
            }));
        }
        
        thread_pool
    }

    pub fn route<T, F>(&mut self, m: Method, p: T, handler: F) 
    where 
        F: HeliumTask,
        T: ToString
    {

        //TODO:
        // Register route in a data structure (Hashmap with String keys and `HeliumTask` values)?
        // When main thread with TCPlistener receives a connection, it looks up the route and adds
        // the corresponding handler to the task queue

        todo!();
    }


    pub fn run(&mut self) -> std::io::Result<()> {

        todo!();
    }
}


/// A HeliumTask represents the behaviour of any task that can be handled by the threadpool.
/// 
/// A blanket implementation is provided for any basic 'route handler' that takes nothing and 
/// returns a serialisable string.
pub trait HeliumTask {
    fn execute(&self) -> TaskResponse;
}

// Blanket implementation for *infallible* route handlers
impl<S: ToString + 'static, T: Fn() -> S> HeliumTask for T {
    fn execute(&self) -> TaskResponse {
        let response_content = self();

        TaskResponse {
            status: StatusCode::Ok,
            content: Some(Box::new(response_content))
        }
    }
}


/// Internal representation of a HeliumTask's completion status
/// 
/// This can be used to represent the execution of a route handler (providing a 
/// statuscode and content) or a generic task run in the task queue, whose status
/// can be represented using HTTP status codes
pub struct TaskResponse {
    status: StatusCode,
    content: Option<Box<dyn ToString>>
}



