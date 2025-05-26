use crate::traits::{HasQuantizationMethods, FiniteFloat};
use std::array::from_fn;

#[derive(Debug, Clone, Copy)]
pub struct Quantizer<T: FiniteFloat, const DIM: usize> {
    a: [T; DIM],            // Lower bounds
    b: [T; DIM],            // Upper bounds
    pub n_levels: [usize; DIM], // ex. DIM=1,N=4: 0,1,2,3
    step_size: [T; DIM],
}

// All intializations
impl<T: FiniteFloat, const DIM: usize> Quantizer<T, DIM> {
    pub fn with_n(a: [T; DIM], b: [T; DIM], n_levels: [usize; DIM]) -> Self {
        let step_size = from_fn(|i| {
            (b[i] - a[i]) / T::from_usize(n_levels[i].saturating_sub(1))
        });
        Quantizer { a, b, n_levels, step_size }
    }

    pub fn with_step_size(a: [T; DIM], b: [T; DIM], step_size: [T; DIM]) -> Self {
        let n_levels = from_fn(|i| {
            ((b[i] - a[i]) / step_size[i] + T::from_usize(1)).to_usize()
        });
        Quantizer { a, b, n_levels, step_size }
    }
}

impl<T: FiniteFloat, const DIM: usize> HasQuantizationMethods<T, DIM> for Quantizer<T, DIM> {
    fn quantize_ieee754(&self, x: [T; DIM]) -> [usize; DIM] {
        from_fn(|i| {
            ((x[i] - self.a[i]) / self.step_size[i]).round_ieee754()
        })
    }

    fn quantize(&self, x: [T; DIM]) -> [usize; DIM] {
        from_fn(|i| {
            ((x[i] - self.a[i]) / self.step_size[i]).round().to_usize()
        })
    }

    fn dequantize(&self, n: [usize; DIM]) -> [T; DIM] {
        from_fn(|i| {
            self.a[i] + T::from_usize(n[i]) * self.step_size[i]
        })
    }
}