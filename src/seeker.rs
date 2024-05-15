use ggez::glam::Vec2;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::graphics;
use ggez::GameResult;

// Defined classes
use crate::{simple_vehicle::SimpleVehicle, vector::Vector};

pub struct Seeker {
    pub vehicle: SimpleVehicle, // should extend a SimpleVehicle
    pub target: Vector,
    pub seek: bool,
    pub touch: bool,
    // steering is actually a static variable, may call for need to adjust
    pub steering: Vector,
}

impl Seeker {
    pub fn new() -> Self {
        Seeker {
            seek: true, 
            touch: false, 
            vehicle: SimpleVehicle::new(Vector::new(0.0, 0.0, 0.0)),
            target: Vector::new(0.0, 0.0, 0.0),
            // is static in the reference implementation 
            steering: Vector::new(0.0, 0.0, 0.0), 
        }
    }

    // update function for Seeker behavior
    pub fn update(&mut self, mut _new_accel: &mut Vector, mut _accel_up: &mut Vector, mut _bank_up: &mut Vector) {
        // apply global force 
        self.steer_for_seek_flee();
        self.vehicle.apply_global_force(self.steering);

        // determine if at target or not
        if self.target.approximate_distance(&mut self.vehicle.local_space.position) <= 0.6 {
            self.touch = true;
        }
        // update the vehicle 
        self.vehicle.update(_new_accel, _accel_up, _bank_up);
    }

    // steering for Seek Flee 
    pub fn steer_for_seek_flee(&mut self) {
        // calculate distance between position and target for seeker and fleer
        // desired velocity 
        if self.seek {
            self.steering.set_diff(self.target, self.vehicle.local_space.position);
        } else {
            self.steering.set_diff(self.vehicle.local_space.position, self.target);
        }

        let goal_length: f32 = 1.1 * self.vehicle.velocity.approximate_length();
        self.steering.set_approximate_truncate(goal_length);

        // subtract velocity from steering
        self.steering.set_diff(self.steering, self.vehicle.velocity);
        self.steering.set_approximate_truncate(self.vehicle.max_force);
    }

    // there are supposed to be some draw functions here, might be good to move them in here for modularity reasons and best practices
    pub fn draw(&mut self, _draw_steering: &mut Vector,_ctx: &mut Context, _canvas: &mut graphics::Canvas, _scale: f32) -> GameResult{
        // draw functions
        let diameter = _scale - 1.0;
        let radius = _scale * 0.5;

        // draw target 
        let(start, finish) = (Vec2::new(self.target.x - diameter, self.target.y), Vec2::new(self.target.x + diameter, self.target.y));
        let horizontal_line = graphics::Mesh::new_line(
            _ctx,
            &[start, finish],
            2.0,
            graphics::Color::BLACK,
        )?;

        let(start2, finish2) = (Vec2::new(self.target.x, self.target.y - diameter), Vec2::new(self.target.x, self.target.y + diameter));
        let vertical_line = graphics::Mesh::new_line(
            _ctx,
            &[start2, finish2],
            2.0,
            graphics::Color::BLACK,
        )?;

        let target = graphics::Mesh::new_circle(
            _ctx, 
            graphics::DrawMode::stroke(2.0), 
            Vec2::new(self.target.x, self.target.y),
            radius, 
            0.2, 
            graphics::Color::BLACK
        )?;
        let mut vehicle_color = graphics::Color::from_rgb(128, 255, 128);
        if self.seek {
            vehicle_color = graphics::Color::from_rgb(128, 255, 128);
        } else {
            vehicle_color = graphics::Color::RED;
        }

        // drawing is only per vehicle, might be unnecessarily resource intensive to have one for both if it's in seeker function
        let vehicle = graphics::Mesh::new_circle(
            _ctx, 
            graphics::DrawMode::fill(), 
            Vec2::new(self.vehicle.local_space.position.x, self.vehicle.local_space.position.y), 
            radius, 
            0.2, 
            vehicle_color
        )?;

        let vehicle_border = graphics::Mesh::new_circle(
            _ctx, 
            graphics::DrawMode::stroke(2.0), 
            Vec2::new(self.vehicle.local_space.position.x, self.vehicle.local_space.position.y), 
            radius, 
            0.2, 
            graphics::Color::BLACK
        )?;

        _canvas.draw(&target, DrawParam::default());
        _canvas.draw(&horizontal_line, DrawParam::default());
        _canvas.draw(&vertical_line, DrawParam::default());

        
        _canvas.draw(&vehicle, DrawParam::default());
        _canvas.draw(&vehicle_border, DrawParam::default());
        
        self.draw_vector(self.steering, _draw_steering, 300.0, _scale, graphics::Color::BLUE, _ctx, _canvas);
        self.draw_vector(self.vehicle.velocity, _draw_steering, 40.0, _scale, graphics::Color::MAGENTA, _ctx, _canvas);
        
        Ok(())
    }

    // need drawSteer passed in from main.rs probably
    pub fn draw_vector(&mut self, _vec: Vector, _draw_steering: &mut Vector, _vscale: f32, _dscale: f32, color: graphics::Color, _ctx: &mut Context, _canvas: &mut graphics::Canvas){
        // for drawing steering and other vectors
        _draw_steering.set_scale(_vscale, _vec);
        _draw_steering.set_sum(self.vehicle.local_space.position, *_draw_steering);
        
        let(start, finish) = (Vec2::new(self.vehicle.local_space.position.x, self.vehicle.local_space.position.y), Vec2::new(_draw_steering.x, _draw_steering.y));
        
        if let Ok(line) = graphics::Mesh::new_line(
            _ctx,
            &[start, finish],
            2.5,
            color,
        ) {
            // println!("start {}", start);
            // println!("finish {}", finish);
            _canvas.draw(&line, DrawParam::default());
        }
    }
}