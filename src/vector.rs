// vector.rs
// random for generating random vectors
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32, // added z for cross product
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    pub fn set(&mut self, vec: Vector) {
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;
    }

    pub fn set_to_zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn magnitude_squared(&self) -> f32{
        return self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
    }

    pub fn magnitude(&self) -> f32 {
        return (self.magnitude_squared()).sqrt();
    }

    // generates random unit vector values for a 3D model, 
    // not sure if this is most efficient
    pub fn gen_random_vector(&mut self){
        let mut rng = rand::thread_rng();
        self.x = rng.gen::<f32>() * 2.0 - 1.0;
        self.y = rng.gen::<f32>() * 2.0 - 1.0;
        self.z = rng.gen::<f32>() * 2.0 - 1.0;

        while self.magnitude_squared() > 1.0 {
            self.x = rng.gen::<f32>() * 2.0 - 1.0;
            self.y = rng.gen::<f32>() * 2.0 - 1.0; 
            self.z = rng.gen::<f32>() * 2.0 - 1.0;
        }
    }

    // approximate length of Vector, this is a fast approximation method
    pub fn approximate_length(&mut self) -> f32{
        // get absolute value of x, y, and z 
        let mut a = self.x.abs(); 
        let mut b = self.y.abs();
        let mut c = self.z.abs();

        // make sure a is the largest coordinate. 
        if a < b {
            let temp = a; 
            a = b;
            b = temp;
        }

        if a < c {
            let temp = a;
            a = c;
            c = temp;
        }

        return (a * 0.9375) + ((b + c) * 0.375);
    }

    // double check because old implementation has distTemp as static variable
    pub fn approximate_distance(&mut self, vec: &mut Vector) -> f32 {
        // disttemp should be difference between this vec
        let mut dist_temp = Vector::new(0.0, 0.0, 0.0);
        dist_temp.set_diff(*self, *vec);
        return dist_temp.approximate_length();
    }

    pub fn set_sum(&mut self, vec: Vector, vec2: Vector) {
        self.x = vec.x + vec2.x;
        self.y = vec.y + vec2.y;
        self.z = vec.z + vec2.z;
    }
    
    pub fn set_diff(&mut self, vec: Vector, vec2: Vector){
        self.x = vec.x - vec2.x;
        self.y = vec.y - vec2.y;
        self.z = vec.z - vec2.z;
    }

    pub fn set_scale(&mut self, scale_factor: f32, vec: Vector) {
        self.x = vec.x * scale_factor;
        self.y = vec.y * scale_factor;
        self.z = vec.z * scale_factor;
    }

    pub fn set_cross(&mut self, vec: Vector, vec2: Vector){
        self.x = (vec.y * vec2.z) - (vec.z * vec2.y);
        self.y = (vec.z * vec2.x) - (vec.x * vec2.z);
        self.z = (vec.x * vec2.y) - (vec.y * vec2.x);
    }

    pub fn set_interpolation(&mut self, blend: f32, vec: &mut Vector, vec2: Vector){
        // blending new acceleration with acceleration
        self.x = vec.x + (blend * (vec2.x - vec.x));
        self.y = vec.y + (blend * (vec2.y - vec.y));
        self.z = vec.z + (blend * (vec2.z - vec.z));
    }

    pub fn set_normalize(&mut self) {
        let mag: f32 = self.magnitude();
        if mag != 0.0 {
            self.set_scale(1.0 / mag, *self);
        }
    }

    // pub fn set_approximate_normalize(&mut self) {
    //     let mag: f32 = self.approximate_length();
    //     if mag != 0.0 {
    //         self.set_scale(1.0 / mag, *self);
    //     }
    // }   

    pub fn set_approximate_truncate(&mut self, threshold: f32) {
        let length: f32 = self.approximate_length();
        if length > threshold {
            self.set_scale(threshold / length, *self);
        }
    }

    // pub fn scale_by(&mut self, scale_factor: f32) {
    //     self.x *= scale_factor;
    //     self.y *= scale_factor;
    //     self.z *= scale_factor;
    // }

    // // fix this to have a return
    // pub fn truncate(&mut self, max_val: f32) {
    //     let mag = self.magnitude();
    //     if mag > max_val {
    //         let scale_factor = max_val / mag;
    //         self.x *= scale_factor;
    //         self.y *= scale_factor;
    //         self.z *= scale_factor;
    //     }
    // }
}

// I added these functions to try and overload but got rid of them during debugging, might still be issues
// impl Add<Vector> for Vector {
//     type Output = Vector;

//     fn add(self, rhs: Vector) -> Vector {
//         Vector {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         }
//     }
// }

// impl Sub<Vector> for Vector {
//     type Output = Vector;

//     fn sub(self, rhs: Vector) -> Vector {
//         Vector {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         }
//     }
// }