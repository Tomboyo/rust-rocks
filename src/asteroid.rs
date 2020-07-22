use rand::Rng;

use crate::entity::Entity;
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

    Entity::new(
        x as i32,
        y as i32,
        rng.gen::<f32>() * MAX_SPEED,
        rng.gen::<f32>() * MAX_SPEED,
        rng.gen::<f32>() * 360.0,
        Sprite::Asteroid)
}
