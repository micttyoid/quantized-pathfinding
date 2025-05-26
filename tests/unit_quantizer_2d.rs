use std::ops::{Add, Sub, Mul, Div};
use quantized_pathfinding::{
    traits::*,
    utils::{
        quantizer::Quantizer,
        quantizer_2d::{
        Quantizer2d, Quantizer2dF32, Quantizer2dF64
        },
    },
};

// common for testing
mod common;
use common::PseudoFloat;
use common::MockQuantizer;


#[cfg(test)]
mod tests {
    use super::*;
    type TestQuantizer2d<T> = Quantizer2d<T>;

    // methods
    #[test]
    fn test_quantizer2d_with_nn_f32() {
        let vec_a = (0.0f32, 0.0f32);
        let vec_b = (10.0f32, 20.0f32);
        let nx = 11;
        let ny = 21;

        let quantizer = TestQuantizer2d::with_nn(vec_a, vec_b, nx, ny);

        assert_eq!(quantizer.quantize((0.0, 0.0)), (0, 0));
        assert_eq!(quantizer.quantize((10.0, 20.0)), (10, 20));
        
        assert_eq!(quantizer.quantize((5.0, 10.0)), (5, 10));
    }

    #[test]
    fn test_quantizer2d_with_nn_f64() {
        let vec_a = (0.0f64, 0.0f64);
        let vec_b = (1.0f64, 1.0f64);
        let nx = 101;
        let ny = 101;

        let quantizer = TestQuantizer2d::with_nn(vec_a, vec_b, nx, ny);

        assert_eq!(quantizer.quantize((0.5, 0.5)), (50, 50));
        assert_eq!(quantizer.quantize((0.25, 0.75)), (25, 75));
    }

    #[test]
    fn test_quantizer2d_with_step_sizes() {
        let vec_a = (0.0f32, 0.0f32);
        let vec_b = (10.0f32, 20.0f32);
        let ss_x = 1.0f32;
        let ss_y = 2.0f32;

        let quantizer = TestQuantizer2d::with_step_sizes(vec_a, vec_b, ss_x, ss_y);

        assert_eq!(quantizer.quantize((3.0, 6.0)), (3, 3));
        assert_eq!(quantizer.quantize((7.5, 15.0)), (8, 8));
    }

    #[test]
    fn test_quantize_dequantize_roundtrip() {
        let vec_a = (0.0f64, 0.0f64);
        let vec_b = (100.0f64, 200.0f64);
        let nx = 101;
        let ny = 201;

        let quantizer = TestQuantizer2d::with_nn(vec_a, vec_b, nx, ny);

        let test_points = vec![
            (0.0, 0.0),
            (50.0, 100.0),
            (100.0, 200.0),
            (25.0, 75.0),
        ];

        for point in test_points {
            let quantized = quantizer.quantize(point);
            let dequantized = quantizer.dequantize(quantized);
            
            assert!((dequantized.0 - point.0).abs() < 1e-10);
            assert!((dequantized.1 - point.1).abs() < 1e-10);
        }
    }

    /* 
    #[test]
    fn test_quantize_clamp() {
        let vec_a = (0.0f32, 0.0f32);
        let vec_b = (10.0f32, 10.0f32);
        let nx = 11;
        let ny = 11;

        let quantizer = TestQuantizer2d::with_nn(vec_a, vec_b, nx, ny);

        let out_of_bounds = quantizer.quantize((15.0, -5.0));
        
        assert!(out_of_bounds.0 <= 10);
        assert!(out_of_bounds.1 <= 10);
    }
    */

    #[test]
    fn test_type_aliases() {
        // Test that type aliases work correctly
        let _quantizer_f32: Quantizer2dF32 = TestQuantizer2d::with_nn(
            (0.0f32, 0.0f32), 
            (1.0f32, 1.0f32), 
            10, 
            10
        );

        let _quantizer_f64: Quantizer2dF64 = TestQuantizer2d::with_nn(
            (0.0f64, 0.0f64), 
            (1.0f64, 1.0f64), 
            10, 
            10
        );

        // If we get here without compilation errors, the type aliases work
        assert!(true);
    }

    #[test]
    fn test_clone_and_copy() {
        let quantizer = TestQuantizer2d::with_nn(
            (0.0f32, 0.0f32), 
            (10.0f32, 10.0f32), 
            11, 
            11
        );

        // Test Clone
        let cloned = quantizer.clone();
        assert_eq!(quantizer.quantize((5.0, 5.0)), cloned.quantize((5.0, 5.0)));

        // Test Copy
        let copied = quantizer;
        assert_eq!(quantizer.quantize((5.0, 5.0)), copied.quantize((5.0, 5.0)));
    }

    #[test]
    fn test_debug_trait() {
        let quantizer = TestQuantizer2d::with_nn(
            (0.0f32, 0.0f32), 
            (10.0f32, 10.0f32), 
            11, 
            11
        );

        // Test that Debug is implemented
        let debug_string = format!("{:?}", quantizer);
        assert!(debug_string.contains("Quantizer2d"));
    }

    #[test]
    fn test_finite_float_trait_f32() {
        // Test FiniteFloat implementation for f32
        let n = 42usize;
        let f = f32::from_usize(n);
        assert_eq!(f, 42.0f32);
        assert_eq!(f.to_usize(), n);

        let rounded = 3.7f32.round();
        assert_eq!(rounded, 4.0f32);

        let rounded2 = 3.2f32.round();
        assert_eq!(rounded2, 3.0f32);
    }

    #[test]
    fn test_finite_float_trait_f64() {
        // Test FiniteFloat implementation for f64
        let n = 123usize;
        let f = f64::from_usize(n);
        assert_eq!(f, 123.0f64);
        assert_eq!(f.to_usize(), n);

        let rounded = 2.8f64.round();
        assert_eq!(rounded, 3.0f64);

        let rounded2 = 2.1f64.round();
        assert_eq!(rounded2, 2.0f64);
    }

    #[test]
    fn test_arithmetic_operations() {
        // Test that FiniteFloat types support required arithmetic
        let a = 5.0f32;
        let b = 3.0f32;

        assert_eq!(a + b, 8.0f32);
        assert_eq!(a - b, 2.0f32);
        assert_eq!(a * b, 15.0f32);
        assert_eq!(a / b, 5.0f32 / 3.0f32);
    }

    #[test]
    fn test_all_round_methods() {
        let quantizer = TestQuantizer2d::with_nn(
            (0.0f32, 0.0f32), 
            (1.0f32, 1.0f32), 
            2, 
            2
        );

        // Test with minimal grid (2x2)
        assert_eq!(quantizer.quantize((0.0, 0.0)), (0, 0));
        assert_eq!(quantizer.quantize((1.0, 1.0)), (1, 1));

        assert_eq!(quantizer.quantize_ieee754((0.5, 0.5)), (0, 0));
        assert_eq!(quantizer.quantize((0.5, 0.5)), (1, 1));
    }    
}
