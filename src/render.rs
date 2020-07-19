use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::entity::Entity;

// TODO: especially evident on asteroids, this renders the background of the
// sprite. We should not render the background pixels of sprites.
pub fn render(
    canvas: &mut Canvas<Window>,
    entity: &Entity,
) -> Result<(), String> {
    let (width, height) = (entity.width(), entity.height());
    let rectangle = Rect::new(
        (entity.x - (width as f32 / 2.0)) as i32,
        (entity.y - (height as f32 / 2.0)) as i32,
        width,
        height);
    canvas.copy_ex(
        entity.texture(),
        None,
        rectangle,
        entity.orientation as f64,
        None, // rotate around center of `rectangle`
        false,
        false)
}
