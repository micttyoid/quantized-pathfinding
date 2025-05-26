use std::ops::{Add, Sub, Mul, Div};
use quantized_pathfinding::{
    traits::*,
    utils::quantizer_3d::*,
};

// common for testing
mod common;
use common::PseudoFloat;
use common::MockQuantizer;

#[cfg(test)]
mod tests {
    use super::*;
    type Quantizer3dPseudo = Quantizer3d<PseudoFloat>;

    #[test]
    fn test_quantizer_3d_f64_with_nnn() {
        let q = Quantizer3d::with_nnn(
            (0.0f64, 0.0f64, 0.0f64),
            (10.0f64, 20.0f64, 30.0f64),
            5, 5, 7
        );

        assert_eq!(q.quantize((0.0, 0.0, 0.0)), (0, 0, 0));
        assert_eq!(q.quantize((10.0, 20.0, 30.0)), (4, 4, 6));
        assert_eq!(q.quantize((5.0, 10.0, 15.0)), (2, 2, 3));
        
        assert_eq!(q.dequantize((0, 0, 0)), (0.0, 0.0, 0.0));
        assert_eq!(q.dequantize((4, 4, 6)), (10.0, 20.0, 30.0));
        assert_eq!(q.dequantize((2, 2, 3)), (5.0, 10.0, 15.0));
    }

    #[test]
    fn test_quantizer_3d_f32_with_walls() {
        let q = Quantizer3d::with_walls(
            (0.0f32, 0.0f32, 0.0f32),
            (1.0f32, 1.0f32, 1.0f32),
            (3, 3, 3)
        );
        
        assert_eq!(q.quantize((0.0, 0.0, 0.0)), (0, 0, 0));
        assert_eq!(q.quantize((0.5, 0.5, 0.5)), (1, 1, 1));
        assert_eq!(q.quantize((1.0, 1.0, 1.0)), (2, 2, 2));
    }

    #[test]
    fn test_quantizer_3d_pseudo_float_with_step_sizes() {
        let q = Quantizer3d::with_step_sizes(
            (PseudoFloat::new(0.0), PseudoFloat::new(0.0), PseudoFloat::new(0.0)),
            (PseudoFloat::new(10.0), PseudoFloat::new(10.0), PseudoFloat::new(10.0)),
            PseudoFloat::new(2.5),
            PseudoFloat::new(2.5),
            PseudoFloat::new(2.5)
        );
        
        assert_eq!(q.quantize((
            PseudoFloat::new(0.0), 
            PseudoFloat::new(2.5), 
            PseudoFloat::new(5.0)
        )), (0, 1, 2));
        
        let result = q.dequantize((0, 1, 2));
        assert_eq!(result, (
            PseudoFloat::new(0.0),
            PseudoFloat::new(2.5),
            PseudoFloat::new(5.0)
        ));
    }

    #[test]
    fn test_quantizer_3d_round_trip_f64() {
        let q = Quantizer3d::with_nnn(
            (-5.0f64, -10.0f64, -15.0f64),
            (5.0f64, 10.0f64, 15.0f64),
            11, 21, 31
        );
        
        for x in 0..11 {
            for y in 0..21 {
                for z in 0..31 {
                    let original = (x, y, z);
                    let dequantized = q.dequantize(original);
                    let requantized = q.quantize(dequantized);
                    assert_eq!(original, requantized);
                }
            }
        }
    }

    #[test]
    fn test_quantizer_3d_round_trip_f32() {
        let q = Quantizer3d::with_nnn(
            (0.0f32, 0.0f32, 0.0f32),
            (1.0f32, 2.0f32, 3.0f32),
            3, 3, 3
        );
        
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    let original = (x, y, z);
                    let dequantized = q.dequantize(original);
                    let requantized = q.quantize(dequantized);
                    assert_eq!(original, requantized);
                }
            }
        }
    }

    #[test]
    fn test_quantizer_3d_pseudo_float_round_trip() {
        let q = Quantizer3d::with_nnn(
            (PseudoFloat::new(0.0), PseudoFloat::new(0.0), PseudoFloat::new(0.0)),
            (PseudoFloat::new(10.0), PseudoFloat::new(20.0), PseudoFloat::new(30.0)),
            5, 5, 5
        );
        
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    let original = (x, y, z);
                    let dequantized = q.dequantize(original);
                    let requantized = q.quantize(dequantized);
                    assert_eq!(original, requantized);
                }
            }
        }
    }

    #[test]
    fn test_quantizer_3d_asymmetric_ranges() {
        let q = Quantizer3d::with_nnn(
            (0.0f64, -5.0f64, 10.0f64),
            (100.0f64, 5.0f64, 50.0f64),
            11, 6, 9
        );
        
        assert_eq!(q.quantize((0.0, -5.0, 10.0)), (0, 0, 0));
        assert_eq!(q.quantize((100.0, 5.0, 50.0)), (10, 5, 8));

        assert_eq!(q.quantize((50.0, 0.0, 30.0)), (5, 3, 4));
    }

    #[test]
    fn test_quantizer_3d_different_precisions() {
        let q_f32 = Quantizer3d::with_nnn(
            (0.0f32, 0.0f32, 0.0f32),
            (1.0f32, 1.0f32, 1.0f32),
            3, 3, 3
        );
        
        let q_f64 = Quantizer3d::with_nnn(
            (0.0f64, 0.0f64, 0.0f64),
            (1.0f64, 1.0f64, 1.0f64),
            3, 3, 3
        );
        
        assert_eq!(q_f32.quantize((0.5, 0.5, 0.5)), (1, 1, 1));
        assert_eq!(q_f64.quantize((0.5, 0.5, 0.5)), (1, 1, 1));
    }

    #[test]
    fn test_quantizer_3d_generic_usage() {
        fn generic_3d_test<T: FiniteFloat>(
            a: (T, T, T), 
            b: (T, T, T), 
            levels: (usize, usize, usize),
            test_point: (T, T, T)
        ) -> (usize, usize, usize) {
            let q = Quantizer3d::with_walls(a, b, levels);
            q.quantize(test_point)
        }
        
        let result_f32 = generic_3d_test(
            (0.0f32, 0.0f32, 0.0f32),
            (10.0f32, 10.0f32, 10.0f32),
            (5, 5, 5),
            (5.0f32, 5.0f32, 5.0f32)
        );
        
        let result_f64 = generic_3d_test(
            (0.0f64, 0.0f64, 0.0f64),
            (10.0f64, 10.0f64, 10.0f64),
            (5, 5, 5),
            (5.0f64, 5.0f64, 5.0f64)
        );
        
        assert_eq!(result_f32, (2, 2, 2));
        assert_eq!(result_f64, (2, 2, 2));
    }

    #[test]
    fn test_quantizer_3d_type_aliases() {
        let q32: Quantizer3dF32 = Quantizer3d::with_nnn(
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
            2, 2, 2
        );
        
        let q64: Quantizer3dF64 = Quantizer3d::with_nnn(
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
            2, 2, 2
        );
        
        let q_pseudo: Quantizer3dPseudo = Quantizer3d::with_nnn(
            (PseudoFloat::new(0.0), PseudoFloat::new(0.0), PseudoFloat::new(0.0)),
            (PseudoFloat::new(1.0), PseudoFloat::new(1.0), PseudoFloat::new(1.0)),
            2, 2, 2
        );
        
        assert_eq!(q32.quantize((0.5, 0.5, 0.5)), (1, 1, 1));
        assert_eq!(q64.quantize((0.5, 0.5, 0.5)), (1, 1, 1));
        assert_eq!(q_pseudo.quantize((
            PseudoFloat::new(0.5), 
            PseudoFloat::new(0.5), 
            PseudoFloat::new(0.5)
        )), (1, 1, 1));
    }

    #[test]
    fn test_quantizer_3d_edge_cases() {
        let q = Quantizer3d::with_nnn(
            (0.0f64, 0.0f64, 0.0f64),
            (1.0f64, 1.0f64, 1.0f64),
            2, 2, 2
        );
        
        assert_eq!(q.quantize((0.0, 0.0, 0.0)), (0, 0, 0));
        assert_eq!(q.quantize((1.0, 1.0, 1.0)), (1, 1, 1));
        
        assert_eq!(q.quantize((-0.1, -0.1, -0.1)), (0, 0, 0));
    }
}
