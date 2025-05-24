
#[derive(Debug, Clone, Copy)]
pub struct Quantizer {
    a: f64,          // Lower bound of input range
    b: f64,          // Upper bound of input range
    pub n_levels: usize, // Number of discrete levels (e.g., N=4 → outputs 0,1,2,3)
    step_size: f64,
}

impl Quantizer {
    pub fn with_n(a: f64, b: f64, n_levels: usize) -> Self {
        let step_size = (b - a) / (n_levels - 1) as f64;
        Quantizer { a, b, n_levels, step_size }
    }

    pub fn with_step_size(a: f64, b: f64, step_size: f64) -> Self {
        let n_levels =  (((b - a) / step_size) + 1.0) as usize; 
        Quantizer { a, b, n_levels, step_size }
    }

    pub fn quantize(&self, x: f64) -> usize {
        ((x - self.a) / self.step_size).round() as usize        
    }

    pub fn dequantize(&self, n: usize) -> f64 {
        self.a + (n as f64) * self.step_size
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quantizer3d {
    pub quantizer_x: Quantizer,
    pub quantizer_y: Quantizer,
    pub quantizer_z: Quantizer,
}

impl Quantizer3d {
    pub fn with_nnn(vec_a:(f64, f64, f64), vec_b: (f64, f64, f64), nx: usize, ny:usize, nz:usize) -> Self {
        Quantizer3d {
            quantizer_x: Quantizer::with_n(vec_a.0,vec_b.0, nx),
            quantizer_y: Quantizer::with_n(vec_a.1,vec_b.1, ny),
            quantizer_z: Quantizer::with_n(vec_a.2,vec_b.2, nz),
        }
    }

    pub fn with_walls(vec_a:(f64, f64, f64), vec_b: (f64, f64, f64), walls: (usize, usize, usize)) -> Self {
        Quantizer3d {
            quantizer_x: Quantizer::with_n(vec_a.0,vec_b.0, walls.0),
            quantizer_y: Quantizer::with_n(vec_a.1,vec_b.1, walls.1),
            quantizer_z: Quantizer::with_n(vec_a.2,vec_b.2, walls.2),
        }
    }    

    pub fn with_step_sizes(vec_a:(f64, f64, f64), vec_b: (f64, f64, f64), ss_x: f64, ss_y:f64, ss_z:f64) -> Self {
        Quantizer3d {
            quantizer_x: Quantizer::with_step_size(vec_a.0, vec_b.0, ss_x),
            quantizer_y: Quantizer::with_step_size(vec_a.1, vec_b.1, ss_y),
            quantizer_z: Quantizer::with_step_size(vec_a.2, vec_b.2, ss_z),
        }
    }

    pub fn quantize(&self, vec:(f64, f64, f64)) -> (usize,usize,usize) {
        (
            self.quantizer_x.quantize(vec.0), 
            self.quantizer_y.quantize(vec.1),
            self.quantizer_z.quantize(vec.2)
        )
    }

    pub fn dequantize(&self, vec_n: (usize,usize,usize)) -> (f64, f64, f64) {
        (
            self.quantizer_x.dequantize(vec_n.0), 
            self.quantizer_y.dequantize(vec_n.1),
            self.quantizer_z.dequantize(vec_n.2)
        )        
    }
}

