use rand::Rng;

use crate::entity::Entity;
use crate::position::Position;
use crate::position::Velocity;
use crate::position::HitMask;
use crate::render::Sprite;

static MAX_SPEED: f32 = 7.0;

pub fn new(
    width: u32,
    height: u32,
) -> Entity {
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

    Entity {
        position: Position { x, y },
        velocity: Velocity {
            dx: rng.gen_range(-MAX_SPEED, MAX_SPEED),
            dy: rng.gen_range(-MAX_SPEED, MAX_SPEED),
        },
        orientation: rng.gen_range(0.0, 360.0),
        sprite: Sprite::Asteroid,
        hitmask: HitMask::Circle {
            radius: 32.0
        },
        timeouts: Vec::new(),
    }
}
