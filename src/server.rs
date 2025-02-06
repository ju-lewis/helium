
use std::{collections::{HashMap, VecDeque}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread};
use crate::http::{self, Method, Path, Query, Request, StatusCode};
use crate::parsing::parse_http_request;

use std::io::{Read, Write};

pub struct Server {
    busy_threads: u32,
    max_threads: u32,
    thread_pool: Vec<thread::JoinHandle<()>>,

    task_queue: Arc<Mutex<VecDeque<TcpStream>>>,
    handlers: HashMap<(Path, Method), Arc<dyn HeliumTask>>
}

impl Server {

    pub fn new(max_threads: u32) -> Self {

        let task_queue = Arc::new(Mutex::new(VecDeque::new()));

        let thread_pool = Self::create_thread_pool(max_threads, Arc::clone(&task_queue));

        Server {
            busy_threads: 0,
            max_threads,
            thread_pool,
            task_queue,
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
                    let mut stream = match tasks.pop_front() {
                        None => continue,
                        Some(s) => s
                    };


                    let mut data = String::new();
                    let read_res = stream.read_to_string(&mut data);
                    if read_res.is_err() {
                        continue;
                    }
                    let req = match parse_http_request(&data) {
                        Ok(r) => r,
                        Err(e) => {

                            println!("There was an error parsing the request: {:?}", e);

                            let response = http::create_response(/* TODO: add error spec to args */);
                            let _ = stream.write(response.as_bytes());
                            continue;
                        }
                    };


                    println!("Received request: {:#?}", req);

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

                    // Add TCP stream to task queue for handling by worker threads

                    let lock_res = self.task_queue.lock();
                    if lock_res.is_err() {
                        continue;
                    }
                    let mut streams = lock_res.unwrap();

                    streams.push_back(s);
                    
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



