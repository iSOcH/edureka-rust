use argminmax::ArgMinMax;

use crate::FindMax;

#[derive(Debug)]
pub struct SimdArgmaxCrate;

impl FindMax for SimdArgmaxCrate {
    fn find_max(&self, data: &[u32]) -> Option<u32> {        
        if data.len() == 0 {
            return None;
        }
        
        let result_index = data.argmax();
        data.get(result_index).cloned()
    }
}