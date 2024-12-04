use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _i in 0..5{
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _j in 0..10{
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}
