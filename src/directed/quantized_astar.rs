//use std::ops::Add;
//use std::hash::Hash;
use std::fmt::Debug;
use pathfinding::directed::astar::astar;
use pathfinding::num_traits::Zero;

use crate::utils::quantizer_3d::Quantizer3d;
use crate::traits::FiniteFloat;

// convenient but less performant than quantized_astar
pub fn quantized_astar_auto<T: FiniteFloat, Cost, FN, IN, FH, FS>(
    quantizer: &Quantizer3d<T>,
    start: (T, T, T),
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<(T,T,T)>, Cost)>
where
    Cost: Zero + Ord + Copy + Debug,
    FN: FnMut(&(T, T, T)) -> IN,
    IN: IntoIterator<Item = ((T, T, T), Cost)>,
    FH: FnMut(&(T, T, T)) -> Cost,
    FS: FnMut(&(T, T, T)) -> bool,    
{
    let start_n: (usize, usize, usize)  = quantizer.quantize(start);
    let result = astar(
        &start_n,
        |n| {
            let dequantized = quantizer.dequantize(*n);
            successors(&dequantized)
                .into_iter()
                .map(|(pos, cost)| (quantizer.quantize(pos), cost))
                .collect::<Vec<_>>()
        },
        |n| heuristic(&quantizer.dequantize(*n)),
        |n| success(&quantizer.dequantize(*n)),
    );
    result.map(|(paths, cost)| {
        let paths_in_float = paths.into_iter()
            .map(|q| quantizer.dequantize(q))
            .collect();
        (paths_in_float, cost)
    })
}

pub fn quantized_astar<T: FiniteFloat, Cost, FN, IN, FH, FS>(
    quantizer: &Quantizer3d<T>,
    start: (T, T, T),
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<(T,T,T)>, Cost)>
where
    Cost: Zero + Ord + Copy + Debug,
    FN: FnMut(&(usize, usize, usize)) -> IN,
    IN: IntoIterator<Item = ((usize, usize, usize), Cost)>,
    FH: FnMut(&(usize, usize, usize)) -> Cost,
    FS: FnMut(&(usize, usize, usize)) -> bool,    
{
    let start_n: (usize, usize, usize)  = quantizer.quantize(start);
    let result = astar(
        &start_n,
        |n| {
            successors(n)
        },
        |n| {
            heuristic(n)
        },
        |n| {
            success(n)
        },
    );
    result.map(|(paths, cost)| {
        let paths_in_float = paths.into_iter()
            .map(|q| quantizer.dequantize(q))
            .collect();
        (paths_in_float, cost)
    })
}
