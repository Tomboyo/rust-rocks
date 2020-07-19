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
                0.0,
                texture)
        })
        .map_err(|e| format!("Cannot create player: {}", e))
}

pub fn handle_event<'a>(
    player: &mut Entity,
    event: &Event,
    texture_creator: &'a TextureCreator<WindowContext>
) -> Option<Entity<'a>> {
    match event {
        Event::LeftJoystick { joystick } => {
            player.dx = joystick.unit_x_axis() * MAX_SPEED;
            player.dy = joystick.unit_y_axis() * MAX_SPEED;
            None
        },
        Event::RightJoystick { joystick } => {
            if let Some(angle) = joystick.angle() {
                player.orientation = angle;
            }
            None
        },
        Event::Fire => Some(fire(
            texture_creator, player
        ).expect("Failed to fire")),
        _ => None
    }
}

pub fn fire<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    player: &Entity
) -> Result<Entity<'a>, String> {
    let x = player.orientation_rad().cos() * player.width() as f32
        + player.x;
    let y = player.orientation_rad().sin() * player.height() as f32
        + player.y;
    let dx = player.orientation_rad().cos() * 10.0;
    let dy = player.orientation_rad().sin() * 10.0;
    
    texture_creator
        .load_texture(Path::new("resources/bullet.bmp"))
        .map(|texture| {
            Entity::new(x as i32, y as i32, dx, dy, player.orientation_deg(), texture)
        })
}
