use rand::Rng;

use crate::entity::Entity;
use crate::position::HitMask;
use crate::render::Sprite;

static MAX_SPEED: f32 = 7.0;

pub fn new(
    bounds: (u32, u32),
) -> Entity {
    let mut rng = rand::thread_rng();
    let (width, height) = bounds;

    // Choose (x, y) on the boundaries of the canvas
    let mut x = 0.0;
    let mut y = 0.0;
    if rng.gen::<f32>() < 0.5 {
        y = rng.gen::<f32>() * height as f32;
    } else {
        x = rng.gen::<f32>() * width as f32;
    }

    Entity {
        x,
        y,
        dx: rng.gen::<f32>() * MAX_SPEED,
        dy: rng.gen::<f32>() * MAX_SPEED,
        orientation: rng.gen::<f32>() * 360.0,
        sprite: Sprite::Asteroid,
        hitmask: HitMask::Circle {
            radius: 32.0
        }
    }
}
