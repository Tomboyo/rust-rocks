use crate::{
    component::{
        HitMask, Orientation, PlayerInput, Position, Sprite, SpriteKind, Thrusters, Velocity,
    },
    resource::bounds::Bounds,
};

// const MAX_SPEED: f32 = 5.0;

pub fn new(
    bounds: &Bounds,
) -> (
    Position,
    Velocity,
    Thrusters,
    Orientation,
    Sprite,
    HitMask,
    PlayerInput,
) {
    (
        Position {
            x: bounds.width / 2.0,
            y: bounds.height / 2.0,
        },
        Velocity { dx: 0.0, dy: 0.0 },
        Thrusters {
            magnitude: 500.0,
            max: 600.0,
        },
        Orientation(0.0),
        Sprite {
            kind: SpriteKind::Player,
        },
        HitMask::Point,
        PlayerInput,
    )
}

// /// Mutate player state based on input events.
// pub fn consume_input(&mut self, controllers: &impl Controllers, event: &Event) {
//     match event {
//         Event::JoyAxisMotion {
//             which, axis_idx, ..
//         } => {
//             self.consume_joystick_input(&controllers.read_joystick(*which, *axis_idx).unwrap());
//         }
//         _ => (),
//     }
// }

// fn consume_joystick_input(&mut self, joystick: &Joystick) {
//     match joystick.which_joystick {
//         WhichJoystick::Left => {
//             self.velocity.dx = joystick.normal_x() * MAX_SPEED;
//             self.velocity.dy = joystick.normal_y() * MAX_SPEED;
//         }
//         WhichJoystick::Right => {
//             if let Some(angle) = joystick.angle() {
//                 self.orientation = angle;
//             }
//         }
//     };
// }
