use reqwest;
use std::sync::mpsc;
use std::thread;

const LINK: &str = "";

// This could be equivalent to the number of requests desired
const NUM_THREADS: usize = 10;

fn main() {
    let (tx, rx) = mpsc::channel();

    for _ in 0..NUM_THREADS {
        let inner_tx = tx.clone();
        thread::spawn(move || {
            inner_tx.send("hey").unwrap();
        });
    }

    for _ in 0..NUM_THREADS {
        println!("Got: {}", rx.recv().unwrap());
    }
}
