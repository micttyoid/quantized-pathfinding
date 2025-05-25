use std::{
    ops::{ Add, Sub, Mul, Div,},
    // convert
};

// Loosen float. Looser than IEEE 754 since it is to work with the quantizer
pub trait FiniteFloat:
    Copy + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> 
{
    fn from_usize(n: usize) -> Self;
    fn to_usize(self) -> usize;
}

impl FiniteFloat for f32 {
    fn from_usize(n: usize) -> Self {
        n as f32
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl FiniteFloat for f64 {
    fn from_usize(n: usize) -> Self {
        n as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}
