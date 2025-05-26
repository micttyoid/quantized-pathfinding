use plotters::prelude::*;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Pillar {
    x_range: [f32; 2],      // Original data coordinates
    y_range: [f32; 2],      // Original data coordinates
    bounds: [f32; 2],       // [width, height] of entire drawing area
    style: ShapeStyle,
}

impl Pillar {
    pub fn new(x_range: [f32; 2], y_range: [f32; 2], bounds: [f32; 2]) -> Self {
        Self {
            x_range,
            y_range,
            bounds,
            style: BLUE.mix(0.2).filled(),
        }
    }

    pub fn with_style(mut self, style: ShapeStyle) -> Self {
        self.style = style;
        self
    }

    pub fn contains(&self, point: [f32; 2]) -> bool {
        point[0] >= self.x_range[0] &&
        point[0] <= self.x_range[1] &&
        point[1] >= self.y_range[0] &&
        point[1] <= self.y_range[1]
    }

    pub fn to_rectangle(&self) -> Rectangle<(f32, f32)> {
        // Normalize coordinates to [0,1] range
        let x1 = self.x_range[0] / self.bounds[0];
        let x2 = self.x_range[1] / self.bounds[0];
        let y1 = self.y_range[0] / self.bounds[1];
        let y2 = self.y_range[1] / self.bounds[1];
        
        Rectangle::new(
            [(x1, y1),  // bottom-left
            (x2, y2)],  // top-right
            self.style,
        )
    }
}

// Create well-spaced pillars
pub fn create_spread_pillars(bounds: [f32; 2], count: usize) -> Vec<Pillar> {
    let width = bounds[0];
    let height = bounds[1];
    let mut pillars = Vec::with_capacity(count);
    
    // Divide area into grid
    let cols = (count as f32).sqrt().ceil() as usize;
    let rows = (count as f32 / cols as f32).ceil() as usize;
    
    let cell_width = width / cols as f32;
    let cell_height = height / rows as f32;
    
    for i in 0..count {
        let col = i % cols;
        let row = i / cols;
        
        let x_start = col as f32 * cell_width + cell_width * 0.1;
        let x_end = (col + 1) as f32 * cell_width - cell_width * 0.1;
        let y_start = row as f32 * cell_height + cell_height * 0.1;
        let y_end = (row + 1) as f32 * cell_height - cell_height * 0.1;
        
        pillars.push(Pillar::new(
            [x_start, x_end],
            [y_start, y_end],
            bounds
        ));
    }
    
    pillars
}


#[derive(Debug, Clone)]
pub struct RandomPillar {
    chambers: Vec<[f32; 4]>, // Each chamber is [x1, x2, y1, y2]
    passages: Vec<[f32; 4]>, // Each passage is [x1, x2, y1, y2]
    bounds: [f32; 2],
    style: ShapeStyle,
}

// Passages do not do much, but it is left here.
impl RandomPillar {
    pub fn new(bounds: [f32; 2]) -> Self {
        let mut rng = rand::rng();
        let mut chambers = Vec::new();
        let mut passages = Vec::new();
        
        // Create 3-5 main chambers
        for _ in 0..rng.random_range(3..=5) {
            let width = bounds[0] * rng.random_range(0.1..0.3);
            let height = bounds[1] * rng.random_range(0.1..0.3);
            let x = rng.random_range(0.0..bounds[0]-width);
            let y = rng.random_range(0.0..bounds[1]-height);
            
            chambers.push([x, x+width, y, y+height]);
        }
        
        // Connect chambers with winding passages
        for i in 0..chambers.len()-1 {
            let start = &chambers[i];
            let end = &chambers[i+1];
            
            // Horizontal then vertical passage
            let mid_x = (start[1] + end[0]) / 2.0;
            passages.push([start[1], mid_x, start[2]+start[3]/2.0-0.02, start[2]+start[3]/2.0+0.02]);
            passages.push([mid_x, mid_x, start[2]+start[3]/2.0, end[2]+end[3]/2.0]);
            passages.push([mid_x, end[0], end[2]+end[3]/2.0-0.02, end[2]+end[3]/2.0+0.02]);
        }
        
        Self {
            chambers,
            passages,
            bounds,
            style: GREEN.mix(0.2).filled(),
        }
    }

    pub fn contains(&self, point: [f32; 2]) -> bool {
        // Check chambers
        for &[x1, x2, y1, y2] in &self.chambers {
            if point[0] >= x1 && point[0] <= x2 &&
               point[1] >= y1 && point[1] <= y2 {
                return true;
            }
        }
        
        // Check passages (with slight padding for easier navigation)
        for &[x1, x2, y1, y2] in &self.passages {
            if point[0] >= x1 - 0.02 && point[0] <= x2 + 0.02 &&
               point[1] >= y1 - 0.02 && point[1] <= y2 + 0.02 {
                return true;
            }
        }
        
        false
    }

    pub fn precise_contains(&self, point: [f32; 2], radius: f32) -> bool {
        for &[x1, x2, y1, y2] in &self.chambers {
            if point[0] + radius >= x1 && point[0] - radius <= x2 &&
               point[1] + radius >= y1 && point[1] - radius <= y2 {
                return true;
            }
        }
        
        // Check passages (precise line segment distance)
        for &[x1, x2, y1, y2] in &self.passages {
            if x1 == x2 { // Vertical passage
                let min_y = y1.min(y2) - radius;
                let max_y = y1.max(y2) + radius;
                if point[0] >= x1 - radius && point[0] <= x1 + radius &&
                   point[1] >= min_y && point[1] <= max_y {
                    return true;
                }
            } else { // Horizontal passage
                let min_x = x1.min(x2) - radius;
                let max_x = x1.max(x2) + radius;
                if point[1] >= y1 - radius && point[1] <= y1 + radius &&
                   point[0] >= min_x && point[0] <= max_x {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn to_elements(&self) -> Vec<Rectangle<(f32, f32)>> {
        let mut elements = Vec::new();
        
        // Add chambers
        for &[x1, x2, y1, y2] in &self.chambers {
            elements.push(Rectangle::new(
                [(x1/self.bounds[0], y1/self.bounds[1]),
                (x2/self.bounds[0], y2/self.bounds[1])],
                self.style,
            ));
        }
        

        // Add passages
        /*
        for &[x1, x2, y1, y2] in &self.passages {
            elements.push(Rectangle::new(
                [(x1/self.bounds[0], y1/self.bounds[1]),
                (x2/self.bounds[0], y2/self.bounds[1])],
                self.style.color.mix(0.8)
                //.mix(0.8), // Slightly different shade
            ));
        }
        */
        
        elements
    }
}

pub fn draw_line_segment(
    start: [f32; 2],
    end: [f32; 2],
    bounds: [f32; 2],
) -> PathElement<(f32, f32)> {
    let normalized_start = (
        start[0] / bounds[0],
        start[1] / bounds[1]
    );
    let normalized_end = (
        end[0] / bounds[0],
        end[1] / bounds[1]
    );

    PathElement::new(
        vec![normalized_start, normalized_end],
        ShapeStyle {
            color: RED.into(),
            filled: true,
            stroke_width: 3,
        }   
    )
}
