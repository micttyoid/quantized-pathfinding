//use std::ops::Add;
use std::hash::Hash;
use std::fmt::Debug;
use pathfinding::directed::astar::astar;
use pathfinding::num_traits::Zero;

use crate::traits::{HasQuantizationMethods, FiniteFloat};

// T - FiniteFloat
// C - Cost
// Q - Quantizer
pub fn quantized_astar<T, C, const DIM: usize, Q, FN, IN, FH, FS>(
    quantizer: &Q,
    start: [T; DIM],
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<[T; DIM]>, C)>
where
    T: FiniteFloat,
    C: Zero + Ord + Copy + Debug,
    Q: HasQuantizationMethods<T, DIM>,
    FN: FnMut(&[usize; DIM]) -> IN,
    IN: IntoIterator<Item = ([usize; DIM], C)>,
    FH: FnMut(&[usize; DIM]) -> C,
    FS: FnMut(&[usize; DIM]) -> bool,
{
    let start_n = quantizer.quantize(start);
    
    let result = astar(
        &start_n,
        |n| successors(n),
        |n| heuristic(n),
        |n| success(n),
    );

    result.map(|(paths, cost)| {
        let paths_in_float = paths.into_iter()
            .map(|quantized| quantizer.dequantize(quantized))
            .collect();
        (paths_in_float, cost)
    })
}
