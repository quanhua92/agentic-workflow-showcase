use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let resource_a = Arc::new(Mutex::new(0));
    let resource_b = Arc::new(Mutex::new(0));

    let a1 = Arc::clone(&resource_a);
    let b1 = Arc::clone(&resource_b);
    let handle1 = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        println!("Thread 1: Locked A, waiting for B...");
        thread::sleep(std::time::Duration::from_millis(50));
        let _b = b1.lock().unwrap();
        println!("Thread 1: Acquired B!");
    });

    let a2 = Arc::clone(&resource_a);
    let b2 = Arc::clone(&resource_b);
    let handle2 = thread::spawn(move || {
        let _b = b2.lock().unwrap();
        println!("Thread 2: Locked B, waiting for A...");
        thread::sleep(std::time::Duration::from_millis(50));
        let _a = a2.lock().unwrap();
        println!("Thread 2: Acquired A!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
