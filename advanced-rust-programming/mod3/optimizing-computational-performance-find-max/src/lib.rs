use std::fmt::Debug;

pub mod single_threaded;
pub mod multi_threaded;

pub trait FindMax : Debug + Send + Sync {
    fn find_max(&self, data: &[u32]) -> Option<u32>;
}