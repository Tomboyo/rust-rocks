use std::ops::Add;
use std::time::Duration;
use std::time::Instant;

use crate::entity::Entity;
use crate::entity::Timeout;
use crate::event::Event;
use crate::position::HitMask;
use crate::render::Sprite;
use crate::render::Textures;

static MAX_SPEED: f32 = 5.0;

pub fn new(
    x: f32,
    y: f32
) -> Entity {
    Entity {
        x,
        y,
        dx: 0.0,
        dy: 0.0,
        orientation: 0.0,
        sprite: Sprite::Player,
        hitmask: HitMask::None,
        timeouts: Vec::new(),
    }
}

pub fn handle_event(
    player: &mut Entity,
    event: &Event,
    textures: &Textures
) -> Option<Entity> {
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
        Event::Fire => Some(fire(player, textures)),
        _ => None
    }
}

pub fn fire(
    player: &Entity,
    textures: &Textures
) -> Entity {
    let (width, height) = textures.dimensions(&player.sprite);

    let x = player.orientation_rad().cos() * width as f32
        + player.x;
    let y = player.orientation_rad().sin() * height as f32
        + player.y;
    let dx = player.orientation_rad().cos() * 10.0;
    let dy = player.orientation_rad().sin() * 10.0;
    
    Entity {
        x, y,
        dx, dy,
        orientation: player.orientation_deg(),
        sprite: Sprite::Bullet,
        hitmask: HitMask::Point,
        timeouts: vec![
            Timeout::Expire {
                when: Instant::now().add(Duration::from_secs(3)),
            }
        ],
    }
}
