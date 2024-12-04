use std::thread;

fn main(){
    let mut handles = Vec::new();

    for i in 1..4{
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("All threads completed.");
}
