use legion::{system, world::SubWorld, IntoQuery};
use sdl2::{controller::Axis, event::Event};

use crate::component::{Orientation, PlayerInput};

use crate::resource::input_events::InputEvents;

/// Holds the last-known values of the controller axis. Because each axis is
/// reported as an event only if it changes, we need this state to determine the
/// updated orientation of the player.
pub struct PlayerInputState {
    normal_x: f32,
    normal_y: f32,
}

impl Default for PlayerInputState {
    fn default() -> Self {
        PlayerInputState {
            normal_x: 0.0,
            normal_y: 0.0,
        }
    }
}

#[system]
#[read_component(PlayerInput)]
#[write_component(Orientation)]
pub fn player_input(
    world: &mut SubWorld,
    #[resource] events: &InputEvents,
    #[state] state: &mut PlayerInputState,
) {
    // We ignore the possibility of two controllers sending input for now.

    let normal_y = events
        .iter()
        .filter(|event| match event {
            Event::ControllerAxisMotion { axis, .. } if *axis == Axis::RightY => true,
            _ => false,
        })
        .map(|event| match event {
            Event::ControllerAxisMotion { value, .. } => normalize_axis(*value),
            _ => panic!("Event should be guaranteed by filter"),
        })
        .next();

    let normal_x = events
        .iter()
        .filter(|event| match event {
            Event::ControllerAxisMotion { axis, .. } if *axis == Axis::RightX => true,
            _ => false,
        })
        .map(|event| match event {
            Event::ControllerAxisMotion { value, .. } => normalize_axis(*value),
            _ => panic!("Event should be guaranteed by filter"),
        })
        .next();

    if let Some(y) = normal_y {
        state.normal_y = y;
    }

    if let Some(x) = normal_x {
        state.normal_x = x;
    }

    if normal_y.is_none() && normal_x.is_none() {
        // no updates this frame, nothing to do.
        return;
    }

    if let Some(angle) = axis_angle(state.normal_x, state.normal_y) {
        // Now that we certainly have an angle, we query for the player
        // orientation
        let mut query = <(&mut Orientation, &PlayerInput)>::query();
        query.for_each_mut(world, move |(orientation, _)| {
            orientation.0 = angle;
        });
    }
}

/// Convert an axis i16 reading to f32 between -1.0 and 1.0 inclusive
fn normalize_axis(value: i16) -> f32 {
    if value >= 0 {
        value as f32 / 32_767.0
    } else {
        value as f32 / 32_768.0
    }
}

// Determines the angle of the joystick, if possible, based on the
// coordinates of the joystick relative to the origin.
//
// The angle cannot be calculated when the coordinates are the origin
// itself. We return None in this case only.
//
// This determines the angle of the joystick by finding the angle between
// the x-axis and the hypotenuse of the special triangle formed by the
// origin and the joystick coordinates.
fn axis_angle(x: f32, y: f32) -> Option<f32> {
    if x == 0.0 && y == 0.0 {
        None
    } else {
        let hypotenuse = ((x * x) + (y * y)).sqrt();
        let degrees = (y / hypotenuse).asin() * (180.0 / std::f32::consts::PI);
        // the y-axis maps -1.0 to -90, 0.0 => 0, and 1.0 => 90. This is the
        // correct angle when x >= 0. When x <= 0, we can subtract that from
        // 180.
        Some(if x <= 0.0 { 180.0 - degrees } else { degrees })
    }
}
