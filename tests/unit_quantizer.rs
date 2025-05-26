use std::ops::{Add, Sub, Mul, Div};
use quantized_pathfinding::{
    traits::*,
    utils::quantizer::*,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantizer_f64_with_n() {
        let q = Quantizer::with_n(0.0f64, 10.0f64, 5);
        assert_eq!(q.n_levels, 5);
        
        assert_eq!(q.quantize(0.0), 0);
        assert_eq!(q.quantize(2.5), 1);
        assert_eq!(q.quantize(5.0), 2);
        assert_eq!(q.quantize(7.5), 3);
        assert_eq!(q.quantize(10.0), 4);
        
        assert_eq!(q.dequantize(0), 0.0);
        assert_eq!(q.dequantize(1), 2.5);
        assert_eq!(q.dequantize(2), 5.0);
        assert_eq!(q.dequantize(3), 7.5);
        assert_eq!(q.dequantize(4), 10.0);
    }

    #[test]
    fn test_quantizer_f32_with_n() {
        let q = Quantizer::with_n(0.0f32, 10.0f32, 5);
        assert_eq!(q.n_levels, 5);
        
        assert_eq!(q.quantize(0.0), 0);
        assert_eq!(q.quantize(2.5), 1);
        assert_eq!(q.quantize(5.0), 2);
        assert_eq!(q.quantize(7.5), 3);
        assert_eq!(q.quantize(10.0), 4);
        
        assert!((q.dequantize(0) - 0.0).abs() < 1e-6);
        assert!((q.dequantize(1) - 2.5).abs() < 1e-6);
        assert!((q.dequantize(2) - 5.0).abs() < 1e-6);
        assert!((q.dequantize(3) - 7.5).abs() < 1e-6);
        assert!((q.dequantize(4) - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_quantizer_f64_with_step_size() {
        let q = Quantizer::with_step_size(0.0f64, 10.0f64, 2.5f64);
        assert_eq!(q.n_levels, 5);
        
        assert_eq!(q.quantize(0.0), 0);
        assert_eq!(q.quantize(2.5), 1);
        assert_eq!(q.quantize(5.0), 2);
        assert_eq!(q.quantize(7.5), 3);
        assert_eq!(q.quantize(10.0), 4);
    }

    #[test]
    fn test_quantizer_f32_with_step_size() {
        let q = Quantizer::with_step_size(0.0f32, 10.0f32, 2.5f32);
        assert_eq!(q.n_levels, 5);
        
        assert_eq!(q.quantize(0.0), 0);
        assert_eq!(q.quantize(2.5), 1);
        assert_eq!(q.quantize(5.0), 2);
        assert_eq!(q.quantize(7.5), 3);
        assert_eq!(q.quantize(10.0), 4);
    }

    #[test]
    fn test_round_trip_f64() {
        let q = Quantizer::with_n(-5.0f64, 5.0f64, 11);
        
        for level in 0..q.n_levels {
            let dequantized = q.dequantize(level);
            let requantized = q.quantize(dequantized);
            assert_eq!(level, requantized);
        }
    }

    #[test]
    fn test_round_trip_f32() {
        let q = Quantizer::with_n(-5.0f32, 5.0f32, 11);
        
        for level in 0..q.n_levels {
            let dequantized = q.dequantize(level);
            let requantized = q.quantize(dequantized);
            assert_eq!(level, requantized);
        }
    }

    #[test]
    fn test_negative_range_f64() {
        let q = Quantizer::with_n(-10.0f64, -5.0f64, 6);
        
        assert_eq!(q.quantize(-10.0), 0);
        assert_eq!(q.quantize(-9.0), 1);
        assert_eq!(q.quantize(-8.0), 2);
        assert_eq!(q.quantize(-7.0), 3);
        assert_eq!(q.quantize(-6.0), 4);
        assert_eq!(q.quantize(-5.0), 5);
    }
    // alias
    #[test]
    fn test_type_aliases() {
        let q32: QuantizerF32 = Quantizer::with_n(0.0, 1.0, 2);
        let q64: QuantizerF64 = Quantizer::with_n(0.0, 1.0, 2);
        
        assert_eq!(q32.quantize(0.5), 1);
        assert_eq!(q64.quantize(0.5), 1);
    }

    // generic
    #[test]
    fn test_generic_usage() {
        let result_f32 = test_generic_quantizer(0.0f32, 10.0f32, 5, 5.0f32);
        let result_f64 = test_generic_quantizer(0.0f64, 10.0f64, 5, 5.0f64);
        
        assert_eq!(result_f32, 2);
        assert_eq!(result_f64, 2);
    }

    #[test]
    fn test_edge_cases() {
        let q = Quantizer::with_n(0.0f64, 1.0f64, 2);
        assert_eq!(q.quantize(-0.1), 0);
    }

    #[test]
    fn test_precision_comparison() {
        let q32 = Quantizer::with_n(0.0f32, 100.0f32, 101);
        let q64 = Quantizer::with_n(0.0f64, 100.0f64, 101);
        
        for i in 0..=100 {
            let val32 = i as f32;
            let val64 = i as f64;
            
            assert_eq!(q32.quantize(val32), q64.quantize(val64));
        }
    }

    // PseudoFloat
    #[test]
    fn test_quantizer_pseudo_float_with_n() {
        let q = Quantizer::with_n(PseudoFloat::new(0.0), PseudoFloat::new(10.0), 5);
        assert_eq!(q.n_levels, 5);
        
        assert_eq!(q.quantize(PseudoFloat::new(0.0)), 0);
        assert_eq!(q.quantize(PseudoFloat::new(2.5)), 1);
        assert_eq!(q.quantize(PseudoFloat::new(5.0)), 2);
        assert_eq!(q.quantize(PseudoFloat::new(7.5)), 3);
        assert_eq!(q.quantize(PseudoFloat::new(10.0)), 4);
        
        assert_eq!(q.dequantize(0), PseudoFloat::new(0.0));
        assert_eq!(q.dequantize(1), PseudoFloat::new(2.5));
        assert_eq!(q.dequantize(2), PseudoFloat::new(5.0));
        assert_eq!(q.dequantize(3), PseudoFloat::new(7.5));
        assert_eq!(q.dequantize(4), PseudoFloat::new(10.0));
    }

    #[test]
    fn test_quantizer_pseudo_float_with_step_size() {
        let q = Quantizer::with_step_size(
            PseudoFloat::new(0.0), 
            PseudoFloat::new(10.0), 
            PseudoFloat::new(2.5)
        );
        assert_eq!(q.n_levels, 5);
        
        assert_eq!(q.quantize(PseudoFloat::new(0.0)), 0);
        assert_eq!(q.quantize(PseudoFloat::new(2.5)), 1);
        assert_eq!(q.quantize(PseudoFloat::new(5.0)), 2);
        assert_eq!(q.quantize(PseudoFloat::new(7.5)), 3);
        assert_eq!(q.quantize(PseudoFloat::new(10.0)), 4);
    }

    #[test]
    fn test_pseudo_float_round_trip() {
        let q = Quantizer::with_n(
            PseudoFloat::new(-5.0), 
            PseudoFloat::new(5.0), 
            11
        );
        
        for level in 0..q.n_levels {
            let dequantized = q.dequantize(level);
            let requantized = q.quantize(dequantized);
            assert_eq!(level, requantized);
        }
    }

    #[test]
    fn test_pseudo_float_negative_range() {
        let q = Quantizer::with_n(
            PseudoFloat::new(-10.0), 
            PseudoFloat::new(-5.0), 
            6
        );
        
        assert_eq!(q.quantize(PseudoFloat::new(-10.0)), 0);
        assert_eq!(q.quantize(PseudoFloat::new(-9.0)), 1);
        assert_eq!(q.quantize(PseudoFloat::new(-8.0)), 2);
        assert_eq!(q.quantize(PseudoFloat::new(-7.0)), 3);
        assert_eq!(q.quantize(PseudoFloat::new(-6.0)), 4);
        assert_eq!(q.quantize(PseudoFloat::new(-5.0)), 5);
    }

    #[test]
    fn test_pseudo_float_fractional_values() {
        let q = Quantizer::with_n(
            PseudoFloat::new(0.0), 
            PseudoFloat::new(1.0), 
            5
        );
        
        assert_eq!(q.quantize(PseudoFloat::new(0.1)), 0);  // Closer to 0.0
        assert_eq!(q.quantize(PseudoFloat::new(0.15)), 1); // Closer to 0.25
        assert_eq!(q.quantize(PseudoFloat::new(0.6)), 2);  // Closer to 0.5
        assert_eq!(q.quantize(PseudoFloat::new(0.9)), 4);  // Closer to 1.0
    }

    #[test]
    fn test_pseudo_float_arithmetic_in_quantizer() {
        let q = Quantizer::with_n(
            PseudoFloat::new(0.0), 
            PseudoFloat::new(10.0), 
            5
        );
        
        let val1 = PseudoFloat::new(2.0);
        let val2 = PseudoFloat::new(0.5);
        let sum = val1 + val2; // Should be 2.5
        
        assert_eq!(q.quantize(sum), 1); // 2.5 should quantize to level 1
    }

    #[test]
    fn test_pseudo_float_generic_quantizer() {
        fn generic_quantize_test<T: FiniteFloat>(a: T, b: T, n_levels: usize, test_val: T) -> usize {
            let q = Quantizer::with_n(a, b, n_levels);
            q.quantize(test_val)
        }
        
        let result = generic_quantize_test(
            PseudoFloat::new(0.0), 
            PseudoFloat::new(10.0), 
            5, 
            PseudoFloat::new(5.0)
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_pseudo_float_precision() {
        let q = Quantizer::with_n(
            PseudoFloat::new(0.0), 
            PseudoFloat::new(100.0), 
            101
        );
        
        for i in 0..=100 {
            let val = PseudoFloat::new(i as f64);
            let quantized = q.quantize(val);
            let dequantized = q.dequantize(quantized);
            assert!((dequantized.value() - val.value()).abs() < 1e-10);
        }
    }

    #[test]
    fn test_pseudo_float_edge_values() {
        let q = Quantizer::with_n(
            PseudoFloat::new(0.0), 
            PseudoFloat::new(1.0), 
            2
        );
        
        assert_eq!(q.quantize(PseudoFloat::new(0.0)), 0);
        assert_eq!(q.quantize(PseudoFloat::new(1.0)), 1);
        assert_eq!(q.quantize(PseudoFloat::new(0.5)), 1); // Should round to nearest

        assert_eq!(q.quantize(PseudoFloat::new(-0.1)), 0); // Should round to 0
    }    
}

fn test_generic_quantizer<T: FiniteFloat>(a: T, b: T, n_levels: usize, test_val: T) -> usize {
    let q = Quantizer::with_n(a, b, n_levels);
    q.quantize(test_val)
}

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
    
    fn round(self) -> Self {
        Self { value: self.value.round() }
    }
}