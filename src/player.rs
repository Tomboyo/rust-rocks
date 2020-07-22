use crate::entity::Entity;
use crate::event::Event;
use crate::render::Sprite;

static MAX_SPEED: f32 = 5.0;

pub fn new(
    position: (i32, i32),
    speed: (f32, f32)
) -> Entity {
    Entity::new(
        position.0, position.1,
        speed.0, speed.1,
        0.0,
        Sprite::Player)
}

pub fn handle_event(
    player: &mut Entity,
    event: &Event,
    render_system: &crate::render::RenderSystem
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
        Event::Fire => Some(fire(player, render_system)),
        _ => None
    }
}

pub fn fire(
    player: &Entity,
    render_system: &crate::render::RenderSystem
) -> Entity {
    let (width, height) = render_system.dimensions(&player.sprite);

    let x = player.orientation_rad().cos() * width as f32
        + player.x;
    let y = player.orientation_rad().sin() * height as f32
        + player.y;
    let dx = player.orientation_rad().cos() * 10.0;
    let dy = player.orientation_rad().sin() * 10.0;
    
    Entity::new(x as i32, y as i32, dx, dy, player.orientation_deg(), Sprite::Bullet)
}
