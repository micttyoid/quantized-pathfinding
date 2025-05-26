use crate::traits::FiniteFloat;
use crate::utils::quantizer::Quantizer;

#[derive(Debug, Clone, Copy)]
pub struct Quantizer2d<T: FiniteFloat> {
    pub quantizer_x: Quantizer<T>,
    pub quantizer_y: Quantizer<T>,
}

impl<T: FiniteFloat> Quantizer2d<T> {
    pub fn with_nn(vec_a: (T, T), vec_b: (T, T), nx: usize, ny: usize) -> Self {
        Quantizer2d {
            quantizer_x: Quantizer::with_n(vec_a.0, vec_b.0, nx),
            quantizer_y: Quantizer::with_n(vec_a.1, vec_b.1, ny),
        }
    }

    pub fn with_walls(vec_a: (T, T), vec_b: (T, T), walls: (usize, usize)) -> Self {
        Quantizer2d {
            quantizer_x: Quantizer::with_n(vec_a.0, vec_b.0, walls.0),
            quantizer_y: Quantizer::with_n(vec_a.1, vec_b.1, walls.1),
        }
    }

    pub fn with_step_sizes(vec_a: (T, T), vec_b: (T, T), ss_x: T, ss_y: T) -> Self {
        Quantizer2d {
            quantizer_x: Quantizer::with_step_size(vec_a.0, vec_b.0, ss_x),
            quantizer_y: Quantizer::with_step_size(vec_a.1, vec_b.1, ss_y),
        }
    }

    /// Quantize with the Round function of truncation of IEEE754
    pub fn quantize_ieee754(&self, vec: (T, T)) -> (usize, usize) {
        (
            self.quantizer_x.quantize_ieee754(vec.0),
            self.quantizer_y.quantize_ieee754(vec.1),
        )
    }

    pub fn quantize(&self, vec: (T, T)) -> (usize, usize) {
        (
            self.quantizer_x.quantize(vec.0),
            self.quantizer_y.quantize(vec.1),
        )
    }

    pub fn dequantize(&self, vec_n: (usize, usize)) -> (T, T) {
        (
            self.quantizer_x.dequantize(vec_n.0),
            self.quantizer_y.dequantize(vec_n.1),
        )
    }
}

// for convenience
pub type Quantizer2dF32 = Quantizer2d<f32>;
pub type Quantizer2dF64 = Quantizer2d<f64>;
