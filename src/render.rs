use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;

pub fn render_texture(
    canvas: &mut Canvas<Window>,
    entity: &Entity,
    texture: &Texture,
) {
    let query = texture.query();
    let rectangle = Rect::new(
        (entity.x - (query.width as f32 / 2.0)) as i32,
        (entity.y - (query.height as f32 / 2.0)) as i32,
        query.width,
        query.height);
    canvas.copy_ex(
        &texture,
        None,
        rectangle,
        entity.orientation,
        None, // rotate around center of `rectangle`
        false,
        false).unwrap();
}

pub fn render_asteroid(
    canvas: &mut Canvas<Window>,
    entity: &Entity
) {
    canvas.set_draw_color(Color::WHITE);
    let rectangle = Rect::new(
        (entity.x - 10.0) as i32,
        (entity.y - 10.0) as i32,
        20,
        20);
    canvas.fill_rect(rectangle).expect("Failed to render entity");
}
