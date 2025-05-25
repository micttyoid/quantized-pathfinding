use crate::traits::FiniteFloat;

#[derive(Debug, Clone, Copy)]
pub struct Quantizer<T: FiniteFloat> {
    a: T,                    // Lower bound of input range
    b: T,                    // Upper bound of input range
    pub n_levels: usize,     // ex. N=4: 0,1,2,3
    step_size: T,
}

impl<T: FiniteFloat> Quantizer<T> {
    pub fn with_n(a: T, b: T, n_levels: usize) -> Self {
        let step_size = (b - a) / T::from_usize(n_levels - 1);
        Quantizer { a, b, n_levels, step_size }
    }

    pub fn with_step_size(a: T, b: T, step_size: T) -> Self {
        let n_levels = ((b - a) / step_size + T::from_usize(1)).to_usize();
        Quantizer { a, b, n_levels, step_size }
    }

    pub fn quantize(&self, x: T) -> usize {
        ((x - self.a) / self.step_size).round().to_usize()
    }

    pub fn dequantize(&self, n: usize) -> T {
        self.a + T::from_usize(n) * self.step_size
    }
}

// for convenience
pub type QuantizerF32 = Quantizer<f32>;
pub type QuantizerF64 = Quantizer<f64>;