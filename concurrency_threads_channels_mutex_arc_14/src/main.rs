use std::sync::mpsc;
use std::thread;
fn main() {
    let mut c = vec![];

    for i in 0..10 {
        c.push(thread::spawn(move || {
            println!("thread #: {}", i);
        }));
    }

    for j in c {
        j.join();
    }

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("vect: {:?}", v);
    });
    handle.join();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    println!("got {}", rx.recv().unwrap());
}
