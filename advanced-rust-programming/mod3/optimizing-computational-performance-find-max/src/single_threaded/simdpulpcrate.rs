use pulp::{bytemuck, Arch, Simd, WithSimd};

use crate::FindMax;

#[derive(Debug)]
pub struct SimdPulpCrate;

impl FindMax for SimdPulpCrate {
    fn find_max(&self, data: &[u32]) -> Option<u32> {
        let arch = Arch::new();
        let result = arch.dispatch(MaxWithPulp(data));
        result
    }
}

struct MaxWithPulp<'a>(&'a [u32]);

impl<'a> WithSimd for MaxWithPulp<'a> {
	type Output = Option<u32>;

	#[inline(always)]
	fn with_simd<S: Simd>(self, simd: S) -> Self::Output {
		let v = self.0;
		let (head, tail) = S::as_simd_u32s(v);
        let mut head_iter = head.iter();

        let max_of_head = if let Some(first) = head_iter.next() {
            let mut max = *first;
            for &c in head_iter {
                let result_mask = simd.greater_than_u32s(max, c);
                max = simd.select_u32s_m32s(result_mask, max, c);
            }
            
            // this seems like some black magic
            let max_slice = std::slice::from_ref(&max);
            let result = bytemuck::cast_slice(max_slice).iter().max().unwrap();
            Some(*result)
        } else {
            None
        };

        max_of_head.iter().chain(tail).max().copied()
    }
}