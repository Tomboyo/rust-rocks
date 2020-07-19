use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::entity::Entity;
use crate::event::Event;

static MAX_SPEED: f32 = 5.0;

pub fn new(
    texture_creator: &TextureCreator<WindowContext>,
    position: (i32, i32),
    speed: (f32, f32)
) -> Result<Entity, String> {
    texture_creator
        .load_texture(Path::new("resources/player-ship.bmp"))
        .map(|texture| {
            Entity::new(
                position.0, position.1,
                speed.0, speed.1,
                texture)
        })
        .map_err(|e| format!("Cannot create player: {}", e))
}

pub fn handle_event(player: &mut Entity, event: &Event) {
    match event {
        Event::LeftJoystick{ joystick } => {
            player.dx = joystick.unit_x_axis() * MAX_SPEED;
            player.dy = joystick.unit_y_axis() * MAX_SPEED;
        },
        Event::RightJoystick{ joystick } => {
            if let Some(angle) = joystick.angle() {
                player.orientation = angle;
            }
        },
        _ => {}
    }
}
