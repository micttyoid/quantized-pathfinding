#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use quantized_pathfinding::traits::*;

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
}