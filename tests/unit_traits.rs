use std::ops::{Add, Sub, Mul, Div};
use quantized_pathfinding::traits::*;

// common for testing
mod common;
use common::PseudoFloat;

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_f32_from_usize() {
        assert_eq!(f32::from_usize(0), 0.0f32);
        assert_eq!(f32::from_usize(42), 42.0f32);
        assert_eq!(f32::from_usize(1000), 1000.0f32);
    }

    #[test]
    fn test_f32_to_usize() {
        assert_eq!(0.0f32.to_usize(), 0);
        assert_eq!(42.0f32.to_usize(), 42);
        assert_eq!(1000.5f32.to_usize(), 1000); // truncation
    }

    #[test]
    fn test_f64_from_usize() {
        assert_eq!(f64::from_usize(0), 0.0f64);
        assert_eq!(f64::from_usize(42), 42.0f64);
        assert_eq!(f64::from_usize(1000), 1000.0f64);
    }

    #[test]
    fn test_f64_to_usize() {
        assert_eq!(0.0f64.to_usize(), 0);
        assert_eq!(42.0f64.to_usize(), 42);
        assert_eq!(1000.7f64.to_usize(), 1000); // truncation
    }

    #[test]
    fn test_arithmetic_operations_f32() {
        let a = f32::from_usize(10);
        let b = f32::from_usize(5);
        
        assert_eq!(a + b, 15.0f32);
        assert_eq!(a - b, 5.0f32);
        assert_eq!(a * b, 50.0f32);
        assert_eq!(a / b, 2.0f32);
    }

    #[test]
    fn test_arithmetic_operations_f64() {
        let a = f64::from_usize(10);
        let b = f64::from_usize(5);
        
        assert_eq!(a + b, 15.0f64);
        assert_eq!(a - b, 5.0f64);
        assert_eq!(a * b, 50.0f64);
        assert_eq!(a / b, 2.0f64);
    }

    #[test]
    fn test_round_trip_conversion_f32() {
        let original = 42usize;
        let float_val = f32::from_usize(original);
        let back_to_usize = float_val.to_usize();
        assert_eq!(original, back_to_usize);
    }

    #[test]
    fn test_round_trip_conversion_f64() {
        let original = 42usize;
        let float_val = f64::from_usize(original);
        let back_to_usize = float_val.to_usize();
        assert_eq!(original, back_to_usize);
    }

    #[test]
    fn test_copy_trait() {
        let a = f32::from_usize(10);
        let b = a; // Copy
        assert_eq!(a, b);
        
        let c = f64::from_usize(20);
        let d = c; // Copy
        assert_eq!(c, d);
    }

    // generic
    fn generic_arithmetic<T: FiniteFloat>(a: usize, b: usize) -> T {
        let x = T::from_usize(a);
        let y = T::from_usize(b);
        x + y * T::from_usize(2) - y
    }

    #[test]
    fn test_generic_usage() {
        let result_f32: f32 = generic_arithmetic(10, 5);
        assert_eq!(result_f32, 15.0f32); // 10 + 5*2 - 5 = 15
        
        let result_f64: f64 = generic_arithmetic(10, 5);
        assert_eq!(result_f64, 15.0f64); // 10 + 5*2 - 5 = 15
    }

    #[test]
    fn test_edge_cases() {
        // zero
        assert_eq!(f32::from_usize(0).to_usize(), 0);
        assert_eq!(f64::from_usize(0).to_usize(), 0);
        
        // large numbers
        let large_num = 1_000_000usize;
        assert_eq!(f32::from_usize(large_num).to_usize(), large_num);
        assert_eq!(f64::from_usize(large_num).to_usize(), large_num);
    }

    #[test]
    fn test_fractional_truncation() {
        // truncation
        assert_eq!(3.14f32.to_usize(), 3);
        assert_eq!(3.99f32.to_usize(), 3);
        assert_eq!(3.14f64.to_usize(), 3);
        assert_eq!(3.99f64.to_usize(), 3);
    }

    
    // Functions using type of trait FiniteFloat
    #[test]
    fn test_basic_arithmetic_f32() {
        let a = 5.5_f32;
        let b = 2.5_f32;
        
        assert_eq!(add_numbers(a, b), 8.0);
        assert_eq!(subtract_numbers(a, b), 3.0);
        assert_eq!(multiply_numbers(a, b), 13.75);
        assert_eq!(divide_numbers(a, b), 2.2);
    }

    #[test]
    fn test_basic_arithmetic_f64() {
        let a = 10.5_f64;
        let b = 3.5_f64;
        
        assert_eq!(add_numbers(a, b), 14.0);
        assert_eq!(subtract_numbers(a, b), 7.0);
        assert_eq!(multiply_numbers(a, b), 36.75);
        assert_eq!(divide_numbers(a, b), 3.0);
    }

    // Type other than f32 and f64
    #[test]
    fn test_basic_arithmetic_pseudo_float() {
        let a = PseudoFloat::new(7.5);
        let b = PseudoFloat::new(2.5);
        
        assert_eq!(add_numbers(a, b), PseudoFloat::new(10.0));
        assert_eq!(subtract_numbers(a, b), PseudoFloat::new(5.0));
        assert_eq!(multiply_numbers(a, b), PseudoFloat::new(18.75));
        assert_eq!(divide_numbers(a, b), PseudoFloat::new(3.0));
    }

    #[test]
    fn test_pseudo_float_operations() {
        let a = PseudoFloat::new(10.5);
        let b = PseudoFloat::new(3.5);

        assert_eq!((a + b).value(), 14.0);
        assert_eq!((a - b).value(), 7.0);
        assert_eq!((a * b).value(), 36.75);
        assert_eq!((a / b).value(), 3.0);

        let result = (a + b) * PseudoFloat::new(2.0) - PseudoFloat::new(1.0);
        assert_eq!(result.value(), 27.0);
    }

    #[test]
    fn test_square_function() {
        assert_eq!(square(4.0_f32), 16.0);
        assert_eq!(square(3.0_f64), 9.0);
        assert_eq!(square(PseudoFloat::new(5.0)), PseudoFloat::new(25.0));
    }

    #[test]
    fn test_sum_range() {
        // 1 + 2 + 3 + 4 + 5 = 15
        assert_eq!(sum_range::<f32>(1, 5), 15.0);
        assert_eq!(sum_range::<f64>(1, 5), 15.0);
        assert_eq!(sum_range::<PseudoFloat>(1, 5), PseudoFloat::new(15.0));
        
        // 0 = 0
        assert_eq!(sum_range::<f32>(0, 0), 0.0);
        
        // 10 + 11 + 12 = 33
        assert_eq!(sum_range::<f64>(10, 12), 33.0);
    }

    #[test]
    fn test_average() {
        let numbers_f32 = vec![1.0_f32, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(average(&numbers_f32), Some(3.0));
        
        let numbers_f64 = vec![2.0_f64, 4.0, 6.0, 8.0];
        assert_eq!(average(&numbers_f64), Some(5.0));
        
        let numbers_pseudo = vec![
            PseudoFloat::new(10.0),
            PseudoFloat::new(20.0),
            PseudoFloat::new(30.0)
        ];
        assert_eq!(average(&numbers_pseudo), Some(PseudoFloat::new(20.0)));
        
        // empty slice
        let empty: Vec<f32> = vec![];
        assert_eq!(average(&empty), None);
    }

    #[test]
    fn test_conversion_functions() {
        // Test from_usize and to_usize
        assert_eq!(f32::from_usize(42), 42.0);
        assert_eq!(f64::from_usize(100), 100.0);
        assert_eq!(PseudoFloat::from_usize(25), PseudoFloat::new(25.0));
        
        assert_eq!((42.7_f32).to_usize(), 42);
        assert_eq!((99.9_f64).to_usize(), 99);
        assert_eq!(PseudoFloat::new(50.8).to_usize(), 50);
    }

    // truncation but through such function also with PseudoFloat
    #[test]
    fn test_convert_and_back() {
        let original_f32 = 15.7_f32;
        let converted_f32 = convert_and_back(original_f32);
        assert_eq!(converted_f32, 15.0);
        
        let original_f64 = 25.9_f64;
        let converted_f64 = convert_and_back(original_f64);
        assert_eq!(converted_f64, 25.0);
        
        let original_pseudo = PseudoFloat::new(35.3);
        let converted_pseudo = convert_and_back(original_pseudo);
        assert_eq!(converted_pseudo, PseudoFloat::new(35.0));
    }

    #[test]
    fn test_combined_operations() {
        fn generic_calculation<T: FiniteFloat>(x: T, y: T) -> T {
            let sum = x + y;
            let product = x * y;
            let difference = sum - product;
            difference / T::from_usize(2)
        }
        
        assert_eq!(generic_calculation(4.0_f32, 2.0_f32), -1.0);
        assert_eq!(generic_calculation(3.0_f64, 2.0_f64), -0.5);
        assert_eq!(
            generic_calculation(PseudoFloat::new(4.0),PseudoFloat::new(2.0)), 
            PseudoFloat::new(-1.0)
        );
    }


    // round methods
    #[test]
    fn test_f32_round() {
        assert_eq!(3.2f32.round(), 3.0f32);
        assert_eq!(3.5f32.round(), 4.0f32);
        assert_eq!(3.7f32.round(), 4.0f32);
        assert_eq!((-3.2f32).round(), -3.0f32);
        assert_eq!((-3.5f32).round(), -4.0f32);
        assert_eq!((-3.7f32).round(), -4.0f32);
        assert_eq!(0.0f32.round(), 0.0f32);
    }

    #[test]
    fn test_f64_round() {
        assert_eq!(3.2f64.round(), 3.0f64);
        assert_eq!(3.5f64.round(), 4.0f64);
        assert_eq!(3.7f64.round(), 4.0f64);
        assert_eq!((-3.2f64).round(), -3.0f64);
        assert_eq!((-3.5f64).round(), -4.0f64);
        assert_eq!((-3.7f64).round(), -4.0f64);
        assert_eq!(0.0f64.round(), 0.0f64);
    }

    #[test]
    fn test_round_with_conversion() {
        // Test round in combination with other trait methods
        let val_f32 = f32::from_usize(5) + 0.7f32;
        assert_eq!(val_f32.round().to_usize(), 6);
        
        let val_f64 = f64::from_usize(5) + 0.3f64;
        assert_eq!(val_f64.round().to_usize(), 5);
    }

    #[test]
    fn test_round_edge_cases() {
        // Test exactly halfway cases (banker's rounding in Rust)
        assert_eq!(2.5f32.round(), 3.0f32);
        assert_eq!(3.5f32.round(), 4.0f32);
        assert_eq!(2.5f64.round(), 3.0f64);
        assert_eq!(3.5f64.round(), 4.0f64);
        
        // Test very small numbers
        assert_eq!(0.1f32.round(), 0.0f32);
        assert_eq!(0.9f32.round(), 1.0f32);
        assert_eq!(0.1f64.round(), 0.0f64);
        assert_eq!(0.9f64.round(), 1.0f64);
    }

    #[test]
    fn test_pseudo_float_round() {
        assert_eq!(PseudoFloat::new(3.2).round(), PseudoFloat::new(3.0));
        assert_eq!(PseudoFloat::new(3.5).round(), PseudoFloat::new(4.0));
        assert_eq!(PseudoFloat::new(3.7).round(), PseudoFloat::new(4.0));
        assert_eq!(PseudoFloat::new(-3.2).round(), PseudoFloat::new(-3.0));
        assert_eq!(PseudoFloat::new(-3.5).round(), PseudoFloat::new(-4.0));
        assert_eq!(PseudoFloat::new(-3.7).round(), PseudoFloat::new(-4.0));
        assert_eq!(PseudoFloat::new(0.0).round(), PseudoFloat::new(0.0));
    }

    #[test]
    fn test_pseudo_float_round_with_conversion() {
        let val = PseudoFloat::from_usize(5) + PseudoFloat::new(0.7);
        assert_eq!(val.round().to_usize(), 6);
        
        let val2 = PseudoFloat::from_usize(5) + PseudoFloat::new(0.3);
        assert_eq!(val2.round().to_usize(), 5);
    }

    #[test]
    fn test_pseudo_float_round_edge_cases() {
        // Test exactly halfway cases
        assert_eq!(PseudoFloat::new(2.5).round(), PseudoFloat::new(3.0));
        assert_eq!(PseudoFloat::new(3.5).round(), PseudoFloat::new(4.0));
        
        // Test very small numbers
        assert_eq!(PseudoFloat::new(0.1).round(), PseudoFloat::new(0.0));
        assert_eq!(PseudoFloat::new(0.9).round(), PseudoFloat::new(1.0));
    }

    #[test]
    fn test_pseudo_float_arithmetic_with_round() {
        let a = PseudoFloat::new(3.7);
        let b = PseudoFloat::new(2.3);
        
        let sum = (a + b).round();
        assert_eq!(sum, PseudoFloat::new(6.0));
        
        let diff = (a - b).round();
        assert_eq!(diff, PseudoFloat::new(1.0));
        
        let product = (a * b).round();
        assert_eq!(product, PseudoFloat::new(9.0)); // 3.7 * 2.3 = 8.51, rounds to 9
    }

    #[test]
    fn test_pseudo_float_generic_usage() {
        fn generic_round_test<T: FiniteFloat>(val: T) -> T {
            val.round()
        }
        
        let pseudo = PseudoFloat::new(3.7);
        let rounded = generic_round_test(pseudo);
        assert_eq!(rounded, PseudoFloat::new(4.0));
    }

    #[test]
    fn test_pseudo_float_round_trip() {
        let original = 42usize;
        let pseudo = PseudoFloat::from_usize(original);
        let rounded = pseudo.round();
        let back_to_usize = rounded.to_usize();
        assert_eq!(original, back_to_usize);
    }


    #[test]
    fn test_finite_float_trait_f32() {
        // Test FiniteFloat implementation for f32
        let n = 42usize;
        let f = f32::from_usize(n);
        assert_eq!(f, 42.0f32);
        assert_eq!(f.to_usize(), n);

        // Test rounding
        let rounded = 3.7f32.round();
        assert_eq!(rounded, 4.0f32);

        let rounded2 = 3.2f32.round();
        assert_eq!(rounded2, 3.0f32);

        // Test IEEE754 rounding (truncation)
        assert_eq!(3.7f32.round_ieee754(), 3);
        assert_eq!(3.2f32.round_ieee754(), 3);
        assert_eq!(3.9f32.round_ieee754(), 3);
    }

    // it is kinda redundant
    #[test]
    fn test_finite_float_trait_f64() {
        // Test FiniteFloat implementation for f64
        let n = 123usize;
        let f = f64::from_usize(n);
        assert_eq!(f, 123.0f64);
        assert_eq!(f.to_usize(), n);

        // Test rounding
        let rounded = 2.8f64.round();
        assert_eq!(rounded, 3.0f64);

        let rounded2 = 2.1f64.round();
        assert_eq!(rounded2, 2.0f64);

        // Test IEEE754 rounding (truncation)
        assert_eq!(2.8f64.round_ieee754(), 2);
        assert_eq!(2.1f64.round_ieee754(), 2);
        assert_eq!(2.9f64.round_ieee754(), 2);
    }    
}

// Basic functions using type of FiniteFloat trait
pub fn add_numbers<T: FiniteFloat>(a: T, b: T) -> T {
    a + b
}

pub fn subtract_numbers<T: FiniteFloat>(a: T, b: T) -> T {
    a - b
}

pub fn multiply_numbers<T: FiniteFloat>(a: T, b: T) -> T {
    a * b
}

pub fn divide_numbers<T: FiniteFloat>(a: T, b: T) -> T {
    a / b
}

pub fn square<T: FiniteFloat>(x: T) -> T {
    x * x
}

pub fn sum_range<T: FiniteFloat>(start: usize, end: usize) -> T {
    let mut result = T::from_usize(0);
    for i in start..=end {
        result = result + T::from_usize(i);
    }
    result
}

pub fn average<T: FiniteFloat>(numbers: &[T]) -> Option<T> {
    if numbers.is_empty() {
        return None;
    }
    
    let mut sum = T::from_usize(0);
    for &num in numbers {
        sum = sum + num;
    }
    
    let count = T::from_usize(numbers.len());
    Some(sum / count)
}

pub fn convert_and_back<T: FiniteFloat>(value: T) -> T {
    let as_usize = value.to_usize();
    T::from_usize(as_usize)
}
