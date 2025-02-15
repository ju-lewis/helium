
use std::{any::Any, collections::{HashMap, VecDeque}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex, mpsc::channel}, thread};
use crate::http::{self, Method, Path, Query, Request, StatusCode};
use crate::parsing::parse_http_request;
use rand::random;

use std::io::{Read, Write};

pub struct Server {
    busy_threads: u32,
    max_threads: u32,
    thread_pool: Vec<thread::JoinHandle<()>>,

    task_queue: Arc<Mutex<VecDeque<(u32, String)>>>,
    streams: HashMap<u32, TcpStream>,
    channel_rx: Option<std::sync::mpsc::Receiver<(u32, String)>>, // Receives (key, content) tuples
    handlers: Arc<HashMap<(Path, Method), Arc<dyn HeliumTask + Sync + Send>>>
}

impl Server {

    pub fn new(max_threads: u32) -> Self {

        let task_queue = Arc::new(Mutex::new(VecDeque::new()));

        let mut server = Server {
            busy_threads: 0,
            max_threads,

            thread_pool: Vec::new(),
            task_queue,
            streams: HashMap::new(),
            channel_rx: None,

            handlers: Arc::new(HashMap::new())
        };

        server.create_thread_pool(max_threads);

        server
    }

    ///
    /// Initialises a Helium `Server` object's thread pool
    ///
    fn create_thread_pool(&mut self, thread_cap: u32) {

        /* Helium multithreaded design philosophy:
         *    The main thread receives TCP connections and reads all data from the stream,
         *    it then adds the TCPStream to a data structure with a key and passes the key
         *    to a worker thread along with the request content. The worker thread parses 
         *    the content and generates a response, before sending the response back to the 
         *    main thread with the corresponding key. The main thread then sends the content 
         *    over the TCPStream.
        */

        let (tx, rx) = channel::<(u32, String)>();
        self.channel_rx = Some(rx);


        for _ in 0..thread_cap {
            let task_mutex = Arc::clone(&self.task_queue);
            let map = Arc::clone(&self.handlers);
            let tx = tx.clone();

            self.thread_pool.push(thread::spawn(move || {
                /* Thread task handler */
                loop {
                    let lock_res = task_mutex.lock();
                    if lock_res.is_err() {
                        return ();
                    }
                    let mut tasks = lock_res.unwrap();
                    
                    // Handle the first task in the queue
                    let (stream_key, request_string) = match tasks.pop_front() {
                        None => continue,
                        Some(s) => s
                    };

                    let req = match parse_http_request(&request_string) {
                        Ok(r) => r,
                        Err(e) => {
                            let response = http::create_response(/* TODO: add error spec to args */);
                            tx.send((stream_key, response));
                            continue;
                        }
                    };

                    // Lookup corresponding HeliumTask
                    let task = match map.get(&(req.path, req.method)) {
                        None => {
                            // Send 404 response
                            tx.send((stream_key, "404 test".to_string()));
                            continue;
                        },
                        Some(t) => t
                    };
                    // Execute task 
                    let response = task.execute();
                    

                    // Return response over TcpStream
                    let status = response.status;
                    let content = match response.content {
                        Some(c) => c.to_string(),
                        None => String::new()
                    };
                        
                    // Format HTTP response
                    tx.send((stream_key, "Test success!".to_string()));
                    
                }
            }));
        }
    }

    pub fn route<F: HeliumTask + 'static + Send + Sync>(&mut self, method: Method, path: Path, handler: F) {

        let temp_handlers = Arc::make_mut(&mut self.handlers);

        temp_handlers.insert((path, method), Arc::new(handler));
    }


    pub fn run(&mut self, socket_addr: &str) -> std::io::Result<()> {


        let listener = TcpListener::bind(socket_addr)?;
        listener.set_nonblocking(true).expect("Couldn't initialise non-blocking TCP listener.");
        
        // Accept incoming connections
        loop {
            // Non-blocking accept
            let stream = listener.accept();

            match stream {
                Err(_) => continue,
                Ok(mut s) => {
                    eprintln!("Received: {:?}", s);
                    // Add TCP stream to task queue for handling by worker threads

                    let lock_res = self.task_queue.lock();
                    if lock_res.is_err() {
                        continue;
                    }
                    let mut tasks = lock_res.unwrap();

                    let mut request_bytes: Vec<u8> = Vec::new();
                    let read_res = s.0.read(&mut request_bytes);

                    if read_res.is_err() {
                        eprintln!("[ERROR]    Couldn't read content from TCPStream.");
                        eprintln!("{:?}", read_res);
                        continue;
                    }

                    let stream_key = rand::random::<u32>();

                    // Log stream in hashmap
                    self.streams.insert(stream_key, s.0);
                    // Pass content and stream key to worker threads
                    let request_string = match String::from_utf8(request_bytes) {
                        Ok(s) => s,
                        Err(_) => continue
                    };
                    tasks.push_back((stream_key, request_string));
                }
            }

            // Now try to receive any messages from the communication channels
            match &self.channel_rx {
                None => continue,
                Some(rx) => {
                    // Non-blocking receive attempt
                    let maybe_data = rx.try_recv();
                    if maybe_data.is_err() {
                        continue;
                    }
                    
                    let data = maybe_data.expect("Channel data errored unexpectedly.");
                    
                    let maybe_stream = self.streams.remove(&data.0);
                    if let Some(mut outgoing_stream) = maybe_stream {
                        let write_res = outgoing_stream.write(data.1.as_bytes());

                        if write_res.is_err() {
                            eprintln!("Failed to write data over socket.");
                            continue;
                        }
                        
                        // Now close the stream
                        let _ = outgoing_stream.shutdown(std::net::Shutdown::Both);
                    }

                }
            }

        }
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



