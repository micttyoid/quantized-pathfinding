use crate::traits::FiniteFloat;
use crate::utils::quantizer::Quantizer;

#[derive(Debug, Clone, Copy)]
pub struct Quantizer3d<T: FiniteFloat> {
    pub quantizer_x: Quantizer<T>,
    pub quantizer_y: Quantizer<T>,
    pub quantizer_z: Quantizer<T>,
}

impl<T: FiniteFloat> Quantizer3d<T> {
    pub fn with_nnn(vec_a: (T, T, T), vec_b: (T, T, T), nx: usize, ny: usize, nz: usize) -> Self {
        Quantizer3d {
            quantizer_x: Quantizer::with_n(vec_a.0, vec_b.0, nx),
            quantizer_y: Quantizer::with_n(vec_a.1, vec_b.1, ny),
            quantizer_z: Quantizer::with_n(vec_a.2, vec_b.2, nz),
        }
    }

    pub fn with_walls(vec_a: (T, T, T), vec_b: (T, T, T), walls: (usize, usize, usize)) -> Self {
        Quantizer3d {
            quantizer_x: Quantizer::with_n(vec_a.0, vec_b.0, walls.0),
            quantizer_y: Quantizer::with_n(vec_a.1, vec_b.1, walls.1),
            quantizer_z: Quantizer::with_n(vec_a.2, vec_b.2, walls.2),
        }
    }

    pub fn with_step_sizes(vec_a: (T, T, T), vec_b: (T, T, T), ss_x: T, ss_y: T, ss_z: T) -> Self {
        Quantizer3d {
            quantizer_x: Quantizer::with_step_size(vec_a.0, vec_b.0, ss_x),
            quantizer_y: Quantizer::with_step_size(vec_a.1, vec_b.1, ss_y),
            quantizer_z: Quantizer::with_step_size(vec_a.2, vec_b.2, ss_z),
        }
    }

    /// Quantize with the Round function of truncation of IEEE754
    pub fn quantize_ieee754(&self, vec: (T, T, T)) -> (usize, usize, usize) {
        (
            self.quantizer_x.quantize_ieee754(vec.0),
            self.quantizer_y.quantize_ieee754(vec.1),
            self.quantizer_z.quantize_ieee754(vec.2),
        )
    }

    pub fn quantize(&self, vec: (T, T, T)) -> (usize, usize, usize) {
        (
            self.quantizer_x.quantize(vec.0),
            self.quantizer_y.quantize(vec.1),
            self.quantizer_z.quantize(vec.2),
        )
    }

    pub fn dequantize(&self, vec_n: (usize, usize, usize)) -> (T, T, T) {
        (
            self.quantizer_x.dequantize(vec_n.0),
            self.quantizer_y.dequantize(vec_n.1),
            self.quantizer_z.dequantize(vec_n.2),
        )
    }
}

// for convenience
pub type Quantizer3dF32 = Quantizer3d<f32>;
pub type Quantizer3dF64 = Quantizer3d<f64>;

