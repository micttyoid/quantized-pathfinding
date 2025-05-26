use quantized_pathfinding::{
    traits::*,
    utils::quantizer::Quantizer,
    directed::*,
};

fn main() {
    let solution = q_astar2d_simple();
    for (v1, v2) in solution {
        println!("({:.2},{:.2}) -> ({:.2},{:.2})", v1[0], v1[1], v2[0], v2[1]);
    }    
}

const N_LEVELS: [usize; 2] = [8, 8];

fn q_astar2d_simple() -> Vec<([f32; 2], [f32; 2])>{
    let vec_a = [0.0, 0.0];  // ~1.42 ~1.42
    let vec_b = [10.0, 10.0];

    let start =  [1.0, 1.0];
    let goal = [9.0, 10.0];

    let blockades: [Blockade; 3] = [
        Blockade {
            x_range: [4.28, 4.30], 
            y_range: [5.70, 5.72],
        },
        Blockade {
            x_range: [2.85, 2.86], 
            y_range: [5.70, 5.72],
        },
        Blockade {
            x_range: [7.13, 7.15], 
            y_range: [8.56, 8.58], 
        },                      
    ];

    let quantizer2d = Quantizer::<f32, 2>::with_n(
       vec_a, vec_b, N_LEVELS
    );
    let goal_n = quantizer2d.quantize(goal);
    let result = quantized_astar(
        &quantizer2d,
        start,
        |&[x, y]| {
            let mut neighbors = vec![];
            for &[dx, dy] in &[
                [1, 0], [-1, 0],
                [0, 1], [0, -1],
            ] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 &&
                   nx < N_LEVELS[0] as i32 && 
                   ny < N_LEVELS[1] as i32 {
                    let mut blocked = false;
                    for blockade in blockades.iter() {
                        if blockade.contains(quantizer2d.dequantize([x,y])) {
                            print!("Blocked! ");
                            blocked = true;
                            break;
                        }
                    }
                    if !blocked {
                        neighbors.push(([nx as usize, ny as usize], 1));
                    }
                }
            }
            neighbors
        },
        |&[x, y]| {
            // heuristic set zero like Dijkstra
            0 as u32
        },
        |&p| p == goal_n, // success condition
    );
    print!("\n");
    let mut drawable_paths: Vec<([f32; 2], [f32; 2])> = vec![];
    match result {
        Some((path, cost)) => {
            println!("Found 2D path with cost {} ({} steps):", cost, path.len());
            let mut pos_old: [f32; 2] = start;
            for (_, pos_new) in path.iter().enumerate() {
                drawable_paths.push((pos_old, *pos_new));
                pos_old = pos_new.clone();                
            } 
        }
        None => println!("No 2D path found"),
    }
    drawable_paths
}

#[derive(Debug, Clone, Copy)]
pub struct Blockade {
    x_range: [f32; 2],
    y_range: [f32; 2],
}

impl Blockade {
    pub fn contains(&self, point: [f32; 2]) -> bool {
        point[0] >= self.x_range[0] &&
        point[0] <= self.x_range[1] &&
        point[1] >= self.y_range[0] &&
        point[1] <= self.y_range[1]
    }
}