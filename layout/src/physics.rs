
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
    
    pub fn add(&mut self, other: Vector2D) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn distance_to(&self, other: Vector2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub struct ForceParams {
    pub charge: f64,       
    pub spring_length: f64, 
    pub spring_stiffness: f64,
    pub damping: f64,      
}

impl Default for ForceParams {
    fn default() -> Self {
        Self {
            charge: -2000.0,
            spring_length: 150.0,
            spring_stiffness: 0.05,
            damping: 0.1,
        }
    }
}

pub fn calculate_repulsion(p1: Vector2D, p2: Vector2D, charge: f64) -> Vector2D {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dist_sq = dx * dx + dy * dy + 0.1; 
    let force = charge / dist_sq;
    
    Vector2D {
        x: dx * force,
        y: dy * force,
    }
}