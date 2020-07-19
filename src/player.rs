use crate::entity::Entity;
use crate::events::Event;

static MAX_SPEED: f32 = 5.0;

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
