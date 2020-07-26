use reqwest;
use std::sync::mpsc;
use std::thread;

const LINK: &str = "http://api.openweathermap.org/data/2.5/weather?q=London,uk&APPID=06f6e2fd036e17bacd53d4a9a658812e";

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
