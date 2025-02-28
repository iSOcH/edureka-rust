use crate::FindMax;

pub mod simdargmaxcrate;
pub mod simdpulpcrate;

pub use simdargmaxcrate::SimdArgmaxCrate;
pub use simdpulpcrate::SimdPulpCrate;

#[derive(Debug)]
pub struct SingleThreadedNaive;

impl FindMax for SingleThreadedNaive {
    fn find_max(&self, data: &[u32]) -> Option<u32> {
        if data.len() == 0 {
            return None;
        }

        let mut max = data[0];

        for i in 1..data.len() {
            let v = data[i];
            if v > max {
                max = v;
            }
        }

        Some(max)
    }
}

#[derive(Debug)]
pub struct StdLib;

impl FindMax for StdLib {
    fn find_max(&self, data: &[u32]) -> Option<u32> {
        let result = data.into_iter().max();
        result.cloned()
    }
}