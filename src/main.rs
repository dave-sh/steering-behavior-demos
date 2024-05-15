// A Demonstration of Seek Flee behavior based on work by Craig Reynolds 
// These are 2D visualizations of 3D steering behaviors 

mod vector;
mod simple_vehicle;
mod seeker;
mod local_space;

// ggez classes
use ggez::graphics::DrawParam;
use ggez::{event, graphics, Context, ContextBuilder, GameResult};
use ggez::event::EventHandler;
use ggez::glam::*;

// Defined classes
use vector::Vector;
use seeker::Seeker;

const WIDTH: f32 = 560.0;
const HEIGHT: f32 = 560.0;

const SCALE: f32 = 15.0;

// seek is seeking one target, flee is fleeing from a different one
// why are there two targets lmfao

fn main() -> GameResult {
    // create a build a context
    let (mut ctx, event_loop) = ContextBuilder::new("SeekFleeDemo", "David Huang")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
        .window_setup(ggez::conf::WindowSetup::default().title("Seek Flee Demo"))
        .build()
        .expect("Could not create context");
    let mut seekflee = SeekFlee::new(&mut ctx);
    seekflee.reset();

    // run
    event::run(ctx, event_loop, seekflee);
}

struct SeekFlee {
    // This is the main state, and references the starting state 
    target_position: Vector,
    // position of seek vehicle
    seek_vehicle: Seeker,
    flee_vehicle: Seeker, 
    frames_since_touch: u32,
    draw_once: bool,
    new_accel: Vector,
    accel_up: Vector,
    bank_up: Vector,
    draw_steer: Vector,
    steering: Vector,
}

impl SeekFlee {
    pub fn new(mut _ctx: &mut Context) -> SeekFlee {
        // calculating middle and assigning target position
        let (mid_width, mid_height) = (WIDTH * 0.5, HEIGHT * 0.5);
        let target_pos = Vector::new(mid_width, mid_height, 0.0);

        // define a view center to make initial position relative to that
        // let view_center = Vector::new(mid_width / (2.0 * SCALE), mid_height / (2.0 * SCALE), 0.0);

        // define two seek vehicles
        let mut seek = Seeker::new();
        let mut flee = Seeker::new();
        flee.seek = false;
        // test

        SeekFlee {
            target_position: target_pos,
            seek_vehicle: seek,
            flee_vehicle: flee,
            frames_since_touch: 0,
            draw_once: true,
            new_accel: Vector::new(0.0, 0.0, 0.0),
            accel_up: Vector::new(0.0, 0.0, 0.0),
            bank_up: Vector::new(0.0, 0.0, 0.0),
            draw_steer: Vector::new(0.0, 0.0, 0.0), // this is used for drawing the steering force and velocity 
            steering: Vector::new(0.0, 0.0, 0.0),
        }
    }

    pub fn reset(&mut self) {
        let mid_width = WIDTH * 0.5;
        let mid_height = HEIGHT * 0.5;
         // define a view center to make initial position relative to that
        let view_center = Vector::new(mid_width,  mid_height, 0.0);
        // initial position should be something random
        // set unit random used to generate a random velocity and position
        // initial position vector
        self.seek_vehicle.vehicle.local_space.position.gen_random_vector();
        // position scaled by 17.0
        self.seek_vehicle.vehicle.local_space.position.set_scale(170.0, self.seek_vehicle.vehicle.local_space.position);
        self.seek_vehicle.vehicle.local_space.position.set_sum(view_center, self.seek_vehicle.vehicle.local_space.position);
        self.seek_vehicle.vehicle.local_space.position.z = 0.0;

        // println!("initial_pos: {:?}", self.seek_vehicle.vehicle.local_space.position);

        // velocity scaled by max Speed
        // set target
        self.seek_vehicle.vehicle.velocity.gen_random_vector();
        self.seek_vehicle.vehicle.velocity.set_scale(self.seek_vehicle.vehicle.max_speed, self.seek_vehicle.vehicle.velocity);
        self.seek_vehicle.vehicle.velocity.z = 0.0;
        self.seek_vehicle.target = self.target_position;
        self.seek_vehicle.touch = false;

        // initialize flee vehicle with identical values
        self.flee_vehicle.seek = false;
        self.flee_vehicle.vehicle.local_space.position.set(self.seek_vehicle.vehicle.local_space.position);
        self.flee_vehicle.vehicle.velocity.set(self.seek_vehicle.vehicle.velocity);
        self.flee_vehicle.target = self.target_position;
        self.flee_vehicle.touch = false;
    }
}

impl EventHandler for SeekFlee {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // update code
        self.seek_vehicle.update(&mut self.new_accel, &mut self.accel_up, &mut self.bank_up);
        self.flee_vehicle.update(&mut self.new_accel, &mut self.accel_up, &mut self.bank_up); 

        //self.flee_vehicle.touch = self.seek_vehicle.touch;

        self.frames_since_touch = if self.seek_vehicle.touch {self.frames_since_touch + 1} else {0};
        if self.frames_since_touch > 15 {
            self.reset();
            self.frames_since_touch = 0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // draw background 
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(230, 230, 153));

        // moved draw functions to seeker.rs
        self.seek_vehicle.draw(&mut self.draw_steer, ctx, &mut canvas, SCALE)?;
        self.flee_vehicle.draw(&mut self.draw_steer, ctx, &mut canvas, SCALE)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}