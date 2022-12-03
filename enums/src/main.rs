use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let data_clone1 = data.clone();
    let t1 = thread::spawn(move || {
        let mut data = data_clone1.lock().unwrap();
        *data += 1;
    });

    let data_clone2 = data.clone();
    let t2 = thread::spawn(move || {
        let mut data = data_clone2.lock().unwrap();
        *data += 1;
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let data = data.lock().unwrap();
    println!("Data: {}", *data);
}
