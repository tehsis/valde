
use valde::bucket_keeper::{BucketKeeper, BucketDefinition};
use std::thread;
use std::io::prelude::*;
use std::time::Duration; 
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};

fn main() {
    let keeper_arc = Arc::new(Mutex::new(BucketKeeper::new(vec![BucketDefinition::new("foo", 10)])));
    
    let keeper = Arc::clone(&keeper_arc);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            let mut keeper = keeper.lock().unwrap();
            println!("Refilling Bucket...");
            keeper.refill("foo");
            println!("[Refill] Tokens left: {}", keeper.get_available_tokens("foo"));

        }
  
    });

    let listener = TcpListener::bind("127.0.0.1:8282").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}