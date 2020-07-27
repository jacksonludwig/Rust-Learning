use reqwest;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

mod json;

const LINK: &str = "";

// This could be equivalent to the number of requests desired
const NUM_THREADS: usize = 10;

fn main() {
    let (tx, rx) = mpsc::channel();
    let request_count = Arc::new(Mutex::new(0));

    for _ in 0..NUM_THREADS {
        let inner_tx = tx.clone();
        let inner_request_count = Arc::clone(&request_count);
        thread::spawn(move || {
            let mut count = inner_request_count.lock().unwrap();
            inner_tx.send("placeholder").unwrap();
            *count += 1;
        });
    }

    for _ in 0..NUM_THREADS {
        println!("Got: {}", rx.recv().unwrap());
    }
    println!(
        "thread::spawn was run: {} times",
        *request_count.lock().unwrap()
    );
    println!("{}", json::read_token());
}
