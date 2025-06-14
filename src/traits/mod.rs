use std::{
    ops::{ Add, Sub, Mul, Div,},
    // convert
};

pub trait HasQuantizationMethods<T, const DIM: usize> {
    fn quantize_ieee754(&self, x: [T; DIM]) -> [usize; DIM];
    fn quantize(&self, x: [T; DIM]) -> [usize; DIM];
    fn dequantize(&self, n: [usize; DIM]) -> [T; DIM];
}

// Loosen float. Looser than IEEE 754 since it is to work with the quantizer
pub trait FiniteFloat:
    Copy + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> 
{
    // conversion
    fn from_usize(n: usize) -> Self;
    fn to_usize(self) -> usize;

    // for quantization or similar operation
    fn round_ieee754 (self) -> usize;
    fn round(self) -> Self;
}

impl FiniteFloat for f32 {
    fn from_usize(n: usize) -> Self {
        n as f32
    }
    
    fn to_usize(self) -> usize {
        self as usize
    }
    
    // IEEE 754 style round
    fn round_ieee754 (self) -> usize {
        self as usize
    }

    fn round(self) -> Self {
        self.round()
    }
}

impl FiniteFloat for f64 {
    fn from_usize(n: usize) -> Self {
        n as f64
    }
    
    fn to_usize(self) -> usize {
        self as usize
    }

    // IEEE 754 style round
    fn round_ieee754 (self) -> usize {
        self as usize
    }
    
    fn round(self) -> Self {
        self.round()
    }
}
