use crate::vector::Vector;
use crate::local_space::LocalSpace;

// DEFAULT VALUES USED
// mass is 1.0F
// max speed is 1.0F
// max force is 0.04F
// acceleration damping value of 0.7
const ACCELERATION_DAMPING: f32 = 0.99;
const GLOBAL_UP: Vector = Vector { x: 0.0, y: 0.1, z: 0.0};
// a lot of static variables from the previous implemetation 
/*
accelUp
bankUp
newAccel
*/

// implementation works fine, values are 

pub struct SimpleVehicle {
    pub local_space: LocalSpace, // extends LocalSpace in old code, but this is how we will include inheritance
    pub mass: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub velocity: Vector,
    pub all_forces: Vector,

    pub acceleration: Vector,
}

impl SimpleVehicle {
    pub fn new(position: Vector) -> Self {
        SimpleVehicle {
            local_space: LocalSpace::new(position),
            mass: 1.0,
            // original values are 0.08 and 0.06
            max_speed: 0.64,
            max_force: 0.48,
            velocity: Vector::new(0.0, 0.0, 0.0),
            all_forces: Vector::new(0.0, 0.0, 0.0),
            acceleration: Vector::new(0.0, 0.0, 0.0),
        }
    }

    pub fn apply_global_force(&mut self, force: Vector){
        self.all_forces.set_sum(self.all_forces, force);
    }

    pub fn update(&mut self, new_accel: &mut Vector, accel_up: &mut Vector, bank_up: &mut Vector){
        // truncate net forces using max forces
        self.all_forces.set_approximate_truncate(self.max_force);

        if self.mass == 1.0 {
            new_accel.set(self.all_forces);
        } else {
            new_accel.set_scale(1.0 / self.mass, self.all_forces);
        }
        self.all_forces.set_to_zero();

        // acceleration should be Interpolated using accelDamping, newAccel, and acceleration
        self.acceleration.set_interpolation(ACCELERATION_DAMPING, new_accel, self.acceleration);
        // add acceleration to velocity 
        self.velocity.set_sum(self.velocity, self.acceleration); // should be acceleration, but only works with new accel?
        // truncate velocity 
        self.velocity.set_approximate_truncate(self.max_speed);
        // add velocity to position 
        self.local_space.position.set_sum(self.local_space.position, self.velocity);

        // banking, but not sure how this works in 2D space
        accel_up.set_scale(0.5, self.acceleration);
        bank_up.set_sum(self.local_space.up, *accel_up);
        bank_up.set_sum(*bank_up, GLOBAL_UP); 
        bank_up.set_normalize(); 

        // println!("Bank Up: x: {} y: {} z: {}", bank_up.x, bank_up.y, bank_up.z);
        // println!("Accel Up: x: {} y: {} z: {}", accel_up.x, accel_up.y, accel_up.z);
        // println!("New Accel: x: {} y: {} z: {}", new_accel.x, new_accel.y, new_accel.z);

        // something to do with local space, but again unsure how this works tbh
        let speed: f32 = self.velocity.magnitude();
        if speed > 0.0 {  
            self.local_space.forward.set_scale(1.0 / speed, self.velocity);
            self.local_space.side.set_cross(self.local_space.forward, *bank_up);
            self.local_space.up.set_cross(self.local_space.side, self.local_space.forward);
        }

    }

    // Deprecated in favor of the update function above
    // pub fn seek(&mut self, target_position: &Vector) {
    //     let desired_velocity = (*target_position - self.position).normalize().scale_by(self.max_speed);
    //     let mut steering = desired_velocity - self.velocity;
    //     steering.truncate(self.max_force);
    //     self.velocity = self.velocity + steering;
    //     self.position = self.position + self.velocity;
    // }
}
