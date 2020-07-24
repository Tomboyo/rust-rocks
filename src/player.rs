use crate::entity::Entity;
use crate::event::Event;
use crate::position::HitMask;
use crate::render::Sprite;
use crate::render::Textures;

static MAX_SPEED: f32 = 5.0;

pub fn new(
    position: (f32, f32),
    speed: (f32, f32)
) -> Entity {
    Entity {
        x: position.0,
        y: position.1,
        dx: speed.0,
        dy: speed.1,
        orientation: 0.0,
        sprite: Sprite::Player,
        hitmask: HitMask::None,
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
        hitmask: HitMask::Point
    }
}
