use std::{thread, time::Duration};

fn main() {
    let thread_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Hello number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..5 {
        println!("Hello number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread_handle.join().unwrap();

    let v = vec![1, 2, 3];
    let thread_handle = thread::spawn(move || {
        println!("Vector v: {:?}", v);
    });
    thread_handle.join().unwrap();
}
