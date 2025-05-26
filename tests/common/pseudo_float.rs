use std::ops::{Add, Sub, Mul, Div};
use quantized_pathfinding::{
    traits::*,
    utils::quantizer::*,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PseudoFloat {
    value: f64,
}

impl PseudoFloat {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Add for PseudoFloat {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self { value: self.value + rhs.value }
    }
}

impl Sub for PseudoFloat {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self { value: self.value - rhs.value }
    }
}

impl Mul for PseudoFloat {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Self { value: self.value * rhs.value }
    }
}

impl Div for PseudoFloat {
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self::Output {
        Self { value: self.value / rhs.value }
    }
}

impl FiniteFloat for PseudoFloat {
    fn from_usize(n: usize) -> Self {
        Self { value: n as f64 }
    }
    
    fn to_usize(self) -> usize {
        self.value as usize
    }

    fn round_ieee754 (self) -> usize {
        self.value as usize
    }    
    
    fn round(self) -> Self {
        Self { value: self.value.round() }
    }
}
