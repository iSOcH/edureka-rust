use std::{num::NonZeroU16, thread};

use crate::FindMax;

#[derive(Debug)]
pub struct Threaded {
    num_threads: NonZeroU16,
    single_thread_implementation: Box<dyn FindMax + Send + Sync>,
}

impl Threaded {
    pub fn new(num_threads: u16, implementation: Box<dyn FindMax + Send + Sync>) -> Threaded {
        Threaded {
            num_threads: num_threads.try_into().expect("pass non-zero number of threads"),
            single_thread_implementation: implementation
        }
    }
}

impl FindMax for Threaded {
    fn find_max(&self, data: &[u32]) -> Option<u32> {
        // thread::scope solves the issue of "borrowed data escapes outside of method" since it ensures the threads are terminated before the method returns
        thread::scope(|scope| {

            let range_length = data.len() / self.num_threads.get() as usize;
            let ranges = data.chunks(range_length);
    
            let thread_handles: Vec<_> = ranges.map(|chunk| {
                scope.spawn(|| self.single_thread_implementation.find_max(chunk))
            }).collect();
    
            let thread_results: Vec<_> = thread_handles.into_iter().map(|t| t.join().expect("unable to join thread")).flatten().collect();
            self.single_thread_implementation.find_max(&thread_results)
        })
    }
}