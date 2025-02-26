use std::{sync::{atomic::AtomicUsize, Arc}, thread, time::Duration};

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut join_handles = Vec::new();

    for _ in 0 .. 10000 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });
        join_handles.push(handle);
    }

    for h in join_handles {
        h.join().unwrap();
    }

    println!("Counter is {}", counter.load(std::sync::atomic::Ordering::Relaxed));
}
