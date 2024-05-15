// 2D local space for boids
use crate::vector::Vector;

pub struct LocalSpace {
    pub forward: Vector, // z direction vector, necessary to do cross product 
    pub side: Vector, // x direction vector
    pub up: Vector, // y direction vector
    
    pub position: Vector, // vector representing position
}

impl LocalSpace {
    pub fn new(initial_position: Vector) -> Self {
        LocalSpace {
            forward: Vector::new(0.0, 0.0, 1.0),
            side: Vector::new(1.0, 0.0, 0.0),
            up: Vector::new(0.0, 1.0, 0.0),
            position: initial_position,
        }
    }

    // a bunch of other functions you may or may not need, but most likely won't at this stage
    // pub fn globalize_direction(&self, direction: Vector) -> Vector {
    //     self.side * direction.x + self.up * direction.y
    // }
}