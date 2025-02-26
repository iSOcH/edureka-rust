use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::{Duration, SystemTime}};

fn main() {
    let started = SystemTime::now();

    let mut experiment = ConcurrentThreads::new();
    experiment.add_thread(10000);
    experiment.add_thread(10000);
    experiment.add_thread(10000);
    experiment.add_thread(10000);
    experiment.add_thread(10000);

    let result = experiment.wait();

    let took = started.elapsed().unwrap();

    print!("Result: {result} after {took:?}");
}

struct ConcurrentThreads {
    counter: Arc<Mutex<i32>>,
    threads: Vec<JoinHandle<()>>
}

impl ConcurrentThreads {
    fn new() -> ConcurrentThreads {
        ConcurrentThreads {
            counter: Arc::new(Mutex::new(0)),
            threads: Vec::new()
        }
    }

    fn add_thread(&mut self, count: i32) -> () {
        let counter_clone = self.counter.clone();

        self.threads.push(thread::spawn(move || {
            for _ in 0..count {
                // sleep before obtaining the lock. if the lock is obtained, it can be released by std::mem::drop on `value_in_lock`
                // or by obtaining the lock in an inner scope
                thread::sleep(Duration::from_millis(1));
                
                let mut value_in_lock = counter_clone.lock().expect("failed to get ref to mutex");
                *value_in_lock += 1;
            }
        }));
    }

    fn wait(&mut self) -> i32 {
        while let Some(t) = self.threads.pop() {
            t.join().unwrap();
        }

        *self.counter.lock().unwrap()
    }
}