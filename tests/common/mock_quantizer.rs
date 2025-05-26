use std::ops::{Add, Sub, Mul, Div};
use quantized_pathfinding::{
    traits::*,
    utils::quantizer::*,
};

// Mock Quantizer
#[derive(Debug, Clone, Copy)]
pub struct MockQuantizer<T: FiniteFloat> {
    min: T,
    max: T,
    step_size: T,
    n_steps: usize,
}

impl<T: FiniteFloat> MockQuantizer<T> {
    pub fn with_n(min: T, max: T, n: usize) -> Self {
        let step_size = (max - min) / T::from_usize(n - 1);
        Self { min, max, step_size, n_steps: n }
    }

    pub fn with_step_size(min: T, max: T, step_size: T) -> Self {
        let n_steps = ((max - min) / step_size).to_usize() + 1;
        Self { min, max, step_size, n_steps }
    }

    pub fn quantize_ieee754(&self, value: T) -> usize {
        let normalized = (value - self.min) / self.step_size;
        let quantized = normalized.round_ieee754();
        quantized.min(self.n_steps - 1)
    }    

    pub fn quantize(&self, value: T) -> usize {
        let normalized = (value - self.min) / self.step_size;
        let quantized = normalized.round().to_usize();
        quantized.min(self.n_steps - 1)
    }

    pub fn dequantize(&self, index: usize) -> T {
        self.min + T::from_usize(index) * self.step_size
    }
}

