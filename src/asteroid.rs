use rand::Rng;

use crate::entity::Entity;
use crate::render::Sprite;

static MAX_SPEED: f32 = 7.0;

pub fn new(
    bounds: (u32, u32),
) -> Entity {
    let mut rng = rand::thread_rng();
    let (width, height) = bounds;

    Entity::new(
        (rng.gen::<f32>() * width as f32) as i32,
        (rng.gen::<f32>() * height as f32) as i32,
        rng.gen::<f32>() * MAX_SPEED,
        rng.gen::<f32>() * MAX_SPEED,
        rng.gen::<f32>() * 360.0,
        Sprite::Asteroid)
}
