use std::ops::{Add, Sub, Mul, Div};
use std::array::from_fn;
use quantized_pathfinding::{
    traits::*,
    utils::quantizer::*,
};

// common for testing
mod common;
use common::PseudoFloat;
use common::MockQuantizer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantizer_1d_with_n() {
        let a = [0.0f32];
        let b = [10.0f32];
        let n_levels = [11]; // 0, 1, 2, ..., 10

        let quantizer = Quantizer::with_n(a, b, n_levels);

        assert_eq!(quantizer.n_levels, [11]);
        
        // Test quantization
        assert_eq!(quantizer.quantize([0.0]), [0]);
        assert_eq!(quantizer.quantize([10.0]), [10]);
        assert_eq!(quantizer.quantize([5.0]), [5]);
        assert_eq!(quantizer.quantize([2.5]), [3]);
        assert_eq!(quantizer.quantize_ieee754([2.5]), [2]); // down

        // Test dequantization
        assert_eq!(quantizer.dequantize([0]), [0.0]);
        assert_eq!(quantizer.dequantize([10]), [10.0]);
        assert_eq!(quantizer.dequantize([5]), [5.0]);
    }

    #[test]
    fn test_quantizer_2d_with_n() {
        let a = [0.0f64, -5.0f64];
        let b = [10.0f64, 5.0f64];
        let n_levels = [11, 21]; // x: 0-10, y: 0-20

        let quantizer = Quantizer::with_n(a, b, n_levels);

        assert_eq!(quantizer.n_levels, [11, 21]);
        
        // Test corner points
        assert_eq!(quantizer.quantize([0.0, -5.0]), [0, 0]);
        assert_eq!(quantizer.quantize([10.0, 5.0]), [10, 20]);
        
        // Test middle point
        assert_eq!(quantizer.quantize([5.0, 0.0]), [5, 10]);
        
        // Test dequantization
        assert_eq!(quantizer.dequantize([0, 0]), [0.0, -5.0]);
        assert_eq!(quantizer.dequantize([10, 20]), [10.0, 5.0]);
    }

    #[test]
    fn test_quantizer_3d_with_n() {
        let a = [0.0f32, 0.0f32, 0.0f32];
        let b = [1.0f32, 2.0f32, 3.0f32];
        let n_levels = [2, 3, 4]; // Minimal levels for each dimension

        let quantizer = Quantizer::with_n(a, b, n_levels);

        assert_eq!(quantizer.n_levels, [2, 3, 4]);
        
        // Test 3D quantization
        assert_eq!(quantizer.quantize([0.0, 0.0, 0.0]), [0, 0, 0]);
        assert_eq!(quantizer.quantize([1.0, 2.0, 3.0]), [1, 2, 3]);
        assert_eq!(quantizer.quantize([0.5, 1.0, 1.5]), [1, 1, 2]);

        assert_eq!(quantizer.quantize_ieee754([0.0, 0.0, 0.0]), [0, 0, 0]);
        assert_eq!(quantizer.quantize_ieee754([1.0, 2.0, 3.0]), [1, 2, 3]);
        assert_eq!(quantizer.quantize_ieee754([0.5, 1.0, 1.5]), [0, 1, 1]);
    }

    #[test]
    fn test_quantizer_with_step_size_1d() {
        let a = [0.0f32];
        let b = [10.0f32];
        let step_size = [1.0f32];

        let quantizer = Quantizer::with_step_size(a, b, step_size);

        // Should have 11 levels (0 to 10 with step 1)
        assert_eq!(quantizer.n_levels, [11]);
        
        // Test quantization with explicit step size
        assert_eq!(quantizer.quantize([3.7]), [4]); // Should round to 4
        assert_eq!(quantizer.quantize([3.2]), [3]); // Should round to 3
    }

    #[test]
    fn test_quantizer_with_step_size_2d() {
        let a = [0.0f64, 0.0f64];
        let b = [10.0f64, 20.0f64];
        let step_size = [2.0f64, 5.0f64];

        let quantizer = Quantizer::with_step_size(a, b, step_size);

        // x: 0, 2, 4, 6, 8, 10 (6 levels)
        // y: 0, 5, 10, 15, 20 (5 levels)
        assert_eq!(quantizer.n_levels, [6, 5]);
        
        assert_eq!(quantizer.quantize([4.0, 10.0]), [2, 2]);
        assert_eq!(quantizer.quantize([6.1, 15.0]), [3, 3]);
    }

    #[test]
    fn test_quantize_dequantize_roundtrip_1d() {
        let a = [0.0f32];
        let b = [100.0f32];
        let n_levels = [101];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        let test_values = [0.0, 25.0, 50.0, 75.0, 100.0];
        
        for &val in &test_values {
            let quantized = quantizer.quantize([val]);
            let dequantized = quantizer.dequantize(quantized);
            
            // Should be very close (within floating point precision)
            assert!((dequantized[0] - val).abs() < 1e-6);
        }
    }

    #[test]
    fn test_quantize_dequantize_roundtrip_2d() {
        let a = [0.0f64, 0.0f64];
        let b = [10.0f64, 20.0f64];
        let n_levels = [11, 21];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        let test_points = [
            [0.0, 0.0],
            [5.0, 10.0],
            [10.0, 20.0],
            [2.0, 4.0],
        ];
        
        for point in test_points {
            let quantized = quantizer.quantize(point);
            let dequantized = quantizer.dequantize(quantized);
            
            for i in 0..2 {
                assert!((dequantized[i] - point[i]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_quantize_ieee754_vs_regular() {
        let a = [0.0f32];
        let b = [10.0f32];
        let n_levels = [11];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test the difference between IEEE754 and regular rounding
        let test_values = [
            2.3f32,  // Should truncate to 2 with IEEE754, round to 2 with regular
            2.7f32,  // Should truncate to 2 with IEEE754, round to 3 with regular
            3.9f32,  // Should truncate to 3 with IEEE754, round to 4 with regular
        ];
        
        for &val in &test_values {
            let regular = quantizer.quantize([val]);
            let ieee754 = quantizer.quantize_ieee754([val]);
            
            // IEEE754 truncates (as usize), regular rounds
            println!("Value: {}, Regular: {:?}, IEEE754: {:?}", val, regular, ieee754);
            
            // For 2.7: regular should be [3], ieee754 should be [2]
            if val == 2.7f32 {
                assert_eq!(regular, [3]);
                assert_eq!(ieee754, [2]);
            }
        }
    }

    // Divide-by-zero does not take place
    #[test]
    fn test_edge_cases_saturating_sub() {
        // Test with n_levels = [1] to check saturating_sub behavior
        let a = [0.0f32];
        let b = [10.0f32];
        let n_levels = [1];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // With only 1 level, step_size should be calculated with saturating_sub
        // (10.0 - 0.0) / (1 - 1) = 10.0 / 0, but saturating_sub makes it (1 - 1) = 0
        // This should handle the division by zero case gracefully
        assert_eq!(quantizer.n_levels, [1]);
    }

    #[test]
    fn test_negative_ranges() {
        let a = [-10.0f32];
        let b = [-1.0f32];
        let n_levels = [10];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        assert_eq!(quantizer.quantize([-10.0]), [0]);
        assert_eq!(quantizer.quantize([-1.0]), [9]);
        assert_eq!(quantizer.quantize([-5.5]), [5]);

        assert_eq!(quantizer.quantize_ieee754([-10.0]), [0]);
        assert_eq!(quantizer.quantize_ieee754([-1.0]), [9]);
        assert_eq!(quantizer.quantize_ieee754([-5.5]), [4]);
    }

    #[test]
    fn test_mixed_positive_negative_2d() {
        let a = [-5.0f64, -10.0f64];
        let b = [5.0f64, 10.0f64];
        let n_levels = [11, 21];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test zero point (should be in the middle)
        assert_eq!(quantizer.quantize([0.0, 0.0]), [5, 10]);
        
        // Test extremes
        assert_eq!(quantizer.quantize([-5.0, -10.0]), [0, 0]);
        assert_eq!(quantizer.quantize([5.0, 10.0]), [10, 20]);
    }

    #[test]
    fn test_high_dimensional_4d() {
        let a = [0.0f32; 4];
        let b = [1.0f32; 4];
        let n_levels = [2; 4]; // Binary quantization in each dimension

        let quantizer = Quantizer::with_n(a, b, n_levels);

        assert_eq!(quantizer.n_levels, [2; 4]);
        
        // Test all corners of 4D hypercube
        assert_eq!(quantizer.quantize([0.0, 0.0, 0.0, 0.0]), [0, 0, 0, 0]);
        assert_eq!(quantizer.quantize([1.0, 1.0, 1.0, 1.0]), [1, 1, 1, 1]);
        assert_eq!(quantizer.quantize([0.3, 0.7, 0.2, 0.8]), [0, 1, 0, 1]);
    }

    #[test]
    fn test_const_generic_flexibility() {
        // Test that the same code works for different dimensions
        
        // 1D
        let q1d = Quantizer::with_n([0.0f32], [1.0f32], [2]);
        assert_eq!(q1d.quantize([0.5]), [1]);

        // 2D  
        let q2d = Quantizer::with_n([0.0f32; 2], [1.0f32; 2], [2; 2]);
        assert_eq!(q2d.quantize([0.5, 0.5]), [1, 1]);

        // 3D
        let q3d = Quantizer::with_n([0.0f32; 3], [1.0f32; 3], [2; 3]);
        assert_eq!(q3d.quantize([0.5, 0.5, 0.5]), [1, 1, 1]);

        // 5D
        let q5d = Quantizer::with_n([0.0f32; 5], [1.0f32; 5], [2; 5]);
        assert_eq!(q5d.quantize([0.5; 5]), [1; 5]);

        // 42D (yes)
        let q42d = Quantizer::with_n([0.0f32; 42], [1.0f32; 42], [2; 42]);
        assert_eq!(q42d.quantize([0.5; 42]), [1; 42]);        
    }

    #[test]
    fn test_clone_and_copy() {
        let a = [0.0f32, 0.0f32];
        let b = [10.0f32, 10.0f32];
        let n_levels = [11, 11];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test Clone
        let cloned = quantizer.clone();
        assert_eq!(quantizer.quantize([5.0, 5.0]), cloned.quantize([5.0, 5.0]));

        // Test Copy
        let copied = quantizer;
        assert_eq!(quantizer.quantize([5.0, 5.0]), copied.quantize([5.0, 5.0]));
    }

    #[test]
    fn test_debug_trait() {
        let a = [0.0f32];
        let b = [10.0f32];
        let n_levels = [11];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test that Debug is implemented
        let debug_string = format!("{:?}", quantizer);
        assert!(debug_string.contains("Quantizer"));
    }

    #[test]
    fn test_precision_regular_vs_ieee754() {
        // regular quantization has better precision
        let a_f64 = [0.0f64];
        let b_f64 = [1.0f64];
        let n_levels = [1000001]; // Very fine quantization

        let quantizer_f64 = Quantizer::with_n(a_f64, b_f64, n_levels);

        let test_val = 0.123456789f64;
   
        let quantized = quantizer_f64.quantize([test_val]);
        let quantized_ieee754 = quantizer_f64.quantize_ieee754([test_val]);

        let dequantized = quantizer_f64.dequantize(quantized);
        let dequantized_ieee754 = quantizer_f64.dequantize(quantized_ieee754);

        assert!((dequantized[0] - test_val).abs() < (dequantized_ieee754[0] - test_val).abs());
    }

    #[test]
    fn test_step_size_calculation() {
        let a = [0.0f32, 0.0f32];
        let b = [10.0f32, 20.0f32];
        let step_size = [0.5f32, 1.0f32];

        let quantizer = Quantizer::with_step_size(a, b, step_size);

        // Check that n_levels is calculated correctly
        // x: 0 to 10 with step 0.5 = 21 levels (0, 0.5, 1.0, ..., 10.0)
        // y: 0 to 20 with step 1.0 = 21 levels (0, 1, 2, ..., 20)
        assert_eq!(quantizer.n_levels, [21, 21]);
    }

    #[test]
    fn test_boundary_conditions() {
        let a = [0.0f32];
        let b = [10.0f32];
        let n_levels = [11];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test values slightly outside bounds
        let slightly_below = quantizer.quantize([-0.1]);
        let slightly_above = quantizer.quantize([10.1]);

        // Values outside bounds will be handled by the quantization formula
        // They might go negative or exceed bounds, depending on implementation
        println!("Below bounds: {:?}", slightly_below);
        println!("Above bounds: {:?}", slightly_above);
    }

    #[test]
    fn test_ieee754_truncation_behavior() {
        let a = [0.0f32];
        let b = [10.0f32];
        let n_levels = [11];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test specific cases where IEEE754 and regular rounding differ
        let test_cases = [
            (1.9f32, 1, 2), // IEEE754: truncate to 1, Regular: round to 2
            (2.1f32, 2, 2), // IEEE754: truncate to 2, Regular: round to 2
            (2.9f32, 2, 3), // IEEE754: truncate to 2, Regular: round to 3
            (3.5f32, 3, 4), // IEEE754: truncate to 3, Regular: round to 4
        ];

        for (value, expected_ieee754, expected_regular) in test_cases {
            let ieee754_result = quantizer.quantize_ieee754([value]);
            let regular_result = quantizer.quantize([value]);

            assert_eq!(ieee754_result[0], expected_ieee754, 
                "IEEE754 quantization failed for value {}", value);
            assert_eq!(regular_result[0], expected_regular, 
                "Regular quantization failed for value {}", value);
        }
    }

    #[test]
    fn test_zero_step_size_edge_case() {
        // Test what happens when a == b (zero range)
        let a = [5.0f32];
        let b = [5.0f32]; // Same as a
        let n_levels = [1];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // All values should quantize to 0 since there's no range
        assert_eq!(quantizer.quantize([5.0]), [0]);
        assert_eq!(quantizer.quantize([0.0]), [0]);
        assert_eq!(quantizer.quantize([10.0]), [0]);
    }

    #[test]
    fn test_large_dimensions() {
        // Test with larger constant generic dimension
        const DIM: usize = 8;
        let a = [0.0f32; DIM];
        let b = [1.0f32; DIM];
        let n_levels = [3; DIM]; // 3 levels per dimension

        let quantizer = Quantizer::with_n(a, b, n_levels);

        assert_eq!(quantizer.n_levels, [3; DIM]);

        // Test corner cases
        let zeros = [0.0f32; DIM];
        let ones = [1.0f32; DIM];
        let halves = [0.5f32; DIM];

        assert_eq!(quantizer.quantize(zeros), [0; DIM]);
        assert_eq!(quantizer.quantize(ones), [2; DIM]);
        assert_eq!(quantizer.quantize(halves), [1; DIM]);
    }

    #[test]
    fn test_asymmetric_ranges() {
        // Test with very different ranges in each dimension
        let a = [0.0f64, -1000.0f64, 0.001f64];
        let b = [1.0f64, 1000.0f64, 0.002f64];
        let n_levels = [11, 21, 11];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test that each dimension is handled independently
        assert_eq!(quantizer.quantize([0.5, 0.0, 0.0015]), [5, 10, 5]);
        assert_eq!(quantizer.quantize([0.0, -1000.0, 0.001]), [0, 0, 0]);
        assert_eq!(quantizer.quantize([1.0, 1000.0, 0.002]), [10, 20, 10]);
    }

    #[test]
    fn test_has_quantization_methods_trait() {
        let a = [0.0f32, 0.0f32];
        let b = [10.0f32, 10.0f32];
        let n_levels = [11, 11];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Test that the trait methods work
        let test_point = [5.0, 7.5];
        
        let quantized_regular = quantizer.quantize(test_point);
        let quantized_ieee754 = quantizer.quantize_ieee754(test_point);
        let dequantized = quantizer.dequantize(quantized_regular);

        assert_eq!(quantized_regular, [5, 8]); // 7.5 rounds to 8
        assert_eq!(quantized_ieee754, [5, 7]); // 7.5 truncates to 7
        assert_eq!(dequantized, [5.0, 8.0]);
    }

    #[test]
    fn test_step_size_precision() {
        // Test that step sizes are calculated correctly
        let a = [0.0f64];
        let b = [1.0f64];
        let step_size = [0.1f64];

        let quantizer = Quantizer::with_step_size(a, b, step_size);

        // Should have 11 levels: 0.0, 0.1, 0.2, ..., 1.0
        assert_eq!(quantizer.n_levels, [11]);

        // Test specific quantization points
        assert_eq!(quantizer.quantize([0.25]), [3]);
        assert_eq!(quantizer.quantize([0.75]), [8]); // round(0.75 * 1.0)

        assert_eq!(quantizer.quantize_ieee754([0.25]), [2]);
        assert_eq!(quantizer.quantize_ieee754([0.75]), [7]); // round(0.75 * 1.0)
    }

    #[test]
    fn test_from_fn_usage() {
        // Test that std::array::from_fn is working correctly in the implementation
        let a = [1.0f32, 2.0f32, 3.0f32];
        let b = [2.0f32, 4.0f32, 6.0f32];
        let n_levels = [2, 3, 4];

        let quantizer = Quantizer::with_n(a, b, n_levels);

        // Each dimension should have the same step sizes
        // 0: (2.0 -1.0)/(2.0 -1.0) = 1.0
        // 1: (4.0 -2.0)/(3.0 -1.0) = 1.0  
        // 2: (6.0 -3.0)/(4.0 -1.0) = 1.0

        let test_point = [2.5, 3.0, 4.5]; // [2.5-1.0, 3.0-2.0, 4.5-3.0]
        let quantized = quantizer.quantize_ieee754(test_point);
        
        // Each should quantize based on their respective ranges
        assert_eq!(quantized, [1, 1, 1]); // Middle points in each dimension
    }
}