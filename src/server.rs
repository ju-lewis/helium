
use std::{collections::{HashMap, VecDeque}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread};
use crate::http::{Method, Path, Query, Request, StatusCode};

pub struct Server {
    busy_threads: u32,
    max_threads: u32,
    thread_pool: Vec<thread::JoinHandle<()>>,

    //task_queue,
    handlers: HashMap<(Path, Method), Arc<dyn HeliumTask>>
}

impl Server {

    pub fn new(max_threads: u32) -> Self {

        let task_queue = VecDeque::new();
        
        let thread_pool = Self::create_thread_pool(max_threads, Arc::new(Mutex::new(task_queue)));

        Server {
            busy_threads: 0,
            max_threads,
            thread_pool,
            handlers: HashMap::new()
        }
    }


    fn create_thread_pool(thread_cap: u32, task_queue: Arc<Mutex<VecDeque<TcpStream>>>) -> Vec<thread::JoinHandle<()>> {
    
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
                    let stream = match tasks.pop_front() {
                        None => continue,
                        Some(s) => s
                    };

                    //TODO: 
                    // - Parse request from TcpStream (obtain path, method, etc.)
                    // - Lookup corresponding HeliumTask
                    // - Execute task 
                    // - Return response over TcpStream

                }
            }));
        }
        thread_pool
    }

    pub fn route<F: HeliumTask + 'static>(&mut self, method: Method, path: Path, handler: F) {

        self.handlers.insert((path, method), Arc::new(handler));
    }


    pub fn run(&mut self, socket_addr: &str) -> std::io::Result<()> {

        let listener = TcpListener::bind(socket_addr)?;

        // Accept incoming connections
        for stream in listener.incoming() {
            match stream {
                Err(_) => continue,
                Ok(s) => {
                    //TODO: Add stream to task queue
                }
            }
        }   

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

// Blanket implementation for simple 'static, infallible' route handlers (don't respond 
// to requests). A use case for a handler like this would be a function that always returns 
// the HTML for an index page.
impl<S: ToString + 'static, T: Fn() -> S> HeliumTask for T {
    fn execute(&self) -> TaskResponse {
        let response_content = self();

        TaskResponse {
            status: StatusCode::Ok,
            content: Some(Box::new(response_content))
        }
    }
}

//impl<S: ToString + 'static, T: Fn(Query) -> S> HeliumTask for T {
//    fn execute(&self) -> TaskResponse {
//        todo!();
//    }
//}


/// Internal representation of a HeliumTask's completion status
/// 
/// This can be used to represent the execution of a route handler (providing a 
/// statuscode and content) or a generic task run in the task queue, whose status
/// can be represented using HTTP status codes
pub struct TaskResponse {
    status: StatusCode,
    content: Option<Box<dyn ToString>>
}



