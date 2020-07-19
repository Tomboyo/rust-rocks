use std::path::Path;

use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::entity::Entity;

static MAX_SPEED: f32 = 7.0;

pub fn new(
    texture_creator: &TextureCreator<WindowContext>,
    bounds: (u32, u32),
) -> Result<Entity, String> {
    let mut rng = rand::thread_rng();
    let (width, height) = bounds;

    texture_creator
        .load_texture(Path::new("resources/asteroid.bmp"))
        .map(|texture| {
            Entity::new(
                (rng.gen::<f32>() * width as f32) as i32,
                (rng.gen::<f32>() * height as f32) as i32,
                rng.gen::<f32>() * MAX_SPEED,
                rng.gen::<f32>() * MAX_SPEED,
                texture)
        })
        .map_err(|e| format!("Cannot create asteroid: {}", e))
}
