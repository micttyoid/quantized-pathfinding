use plotters::{
    prelude::*,
    coord::types::RangedCoordf32,
};

use quantized_pathfinding::{
    traits::*,
    utils::quantizer::Quantizer,
    directed::*,
};
use rand::Rng;

mod helpers;
use helpers::*;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut rng = rand::rng();

    let drawing_area: [f32; 2] = [640.0, 640.0];
    let start: [f32; 2] = [5.0, 5.0];
    let goal: [f32; 2] = [635.0, 635.0];

    let pillars = create_spread_pillars(drawing_area, rng.random_range(5..42));
    let rand_pillars = RandomPillar::new(drawing_area);

    let solution = q_astar2d(
        start, goal, drawing_area,
        pillars.clone(), rand_pillars.clone()
    );

    let root = BitMapBackend::new("maze2d-solution.png", (drawing_area[0] as u32, drawing_area[1] as u32)).into_drawing_area();
    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..drawing_area[0] as i32, 0..drawing_area[0] as i32),
    ));
    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
    };    
    root.fill(&RGBColor(240, 200, 200))?;

    let pillars = pillars;
    for p in pillars {
        root.draw(&p.to_rectangle())?;
    }
    for rp in rand_pillars.to_elements() {
        root.draw(&rp).unwrap();
    }

    root.draw(&dot_and_label(start[0]/drawing_area[0], start[0]/drawing_area[1]))?;
    root.draw(&dot_and_label(goal[0]/drawing_area[0], goal[0]/drawing_area[1]))?;
    root.present()?;    
    for (v1, v2) in solution {
        println!("({:.2},{:.2}) -> ({:.2},{:.2})", v1[0], v1[1], v2[0], v2[1]);
        root.draw(&draw_line_segment(v1, v2, drawing_area))?;
    }
    Ok(())
}

const N_LEVELS: [usize; 2] = [25, 25];

fn q_astar2d(
    start: [f32; 2], goal: [f32; 2], bounds: [f32; 2],
    pillars: Vec<Pillar>, rand_pillars: RandomPillar,
) -> Vec<([f32; 2], [f32; 2])>{
    let vec_a = [0.0, 0.0];
    let vec_b = bounds; 
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
                   ny < N_LEVELS[1] as i32 && 
                   !rand_pillars.contains(quantizer2d.dequantize([x,y]))
                   {
                    let mut blocked = false;
                    for pillar in pillars.iter() {
                        if pillar.contains(quantizer2d.dequantize([x,y])) {
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

    let mut drawable_paths: Vec<([f32; 2], [f32; 2])> = vec![];
    match result {
        Some((path, cost)) => {
            println!("Found 3D path with cost {} ({} steps):", cost, path.len());
            let mut pos_old: [f32; 2] = start;
            for (_, pos_new) in path.iter().enumerate() {
                drawable_paths.push((pos_old, *pos_new));
                pos_old = pos_new.clone();                
            } 
        }
        None => println!("No 3D path found"),
    }
    drawable_paths
}
