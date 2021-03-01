use rand::Rng;

use crate::position::HitMask;
use crate::position::Position;
use crate::position::Velocity;
use crate::position::{Collidable, DynamicPosition};
use crate::render::Renderable;
use crate::render::Sprite;

static MAX_SPEED: f32 = 7.0;

pub struct Asteroid {
    pub position: Position,
    pub velocity: Velocity,
    pub orientation: f32, // in degrees
    pub sprite: Sprite,
    pub hitmask: HitMask,
}

pub fn new(width: u32, height: u32) -> Asteroid {
    let mut rng = rand::thread_rng();

    // Choose (x, y) on the boundaries of the canvas
    let mut x = 0.0;
    let mut y = 0.0;
    if rng.gen::<f32>() < 0.5 {
        y = rng.gen::<f32>() * height as f32;
        if rng.gen::<f32>() < 0.5 {
            x = width as f32;
        }
    } else {
        x = rng.gen::<f32>() * width as f32;
        if rng.gen::<f32>() < 0.5 {
            y = height as f32;
        }
    }

    Asteroid {
        position: Position { x, y },
        velocity: Velocity {
            dx: rng.gen_range(-MAX_SPEED, MAX_SPEED),
            dy: rng.gen_range(-MAX_SPEED, MAX_SPEED),
        },
        orientation: rng.gen_range(0.0, 360.0),
        sprite: Sprite::Asteroid,
        hitmask: HitMask::Circle { radius: 32.0 },
    }
}

impl Collidable for Asteroid {
    fn hit_mask(&self) -> &HitMask {
        &self.hitmask
    }

    fn position(&self) -> &Position {
        &self.position
    }
}

impl<'a> Renderable<'a> for Asteroid {
    fn position(&'a self) -> &'a Position {
        &self.position
    }

    fn orientation(&self) -> f64 {
        self.orientation as f64
    }

    fn sprite(&'a self) -> &'a Sprite {
        &self.sprite
    }
}

impl DynamicPosition for Asteroid {
    fn translate(&mut self) {
        self.position.translate(&self.velocity);
    }

    fn clamp(&mut self, modx: f32, mody: f32) {
        self.position.modulate(modx, mody);
    }
}
