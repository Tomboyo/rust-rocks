use legion::{system, world::SubWorld, IntoQuery};
use sdl2::{controller::Axis, event::Event};

use crate::{
    component::{Orientation, PlayerInput, Thrusters, Velocity},
    resource::delta_time::DeltaTime,
};

use crate::resource::input_events::InputEvents;

/// Holds the last-known values of the controller axis. Because each axis is
/// reported as an event only if it changes, we need this state to determine the
/// updated orientation of the player.
pub struct PlayerInputState {
    right_normal_x: f32,
    right_normal_y: f32,
    left_normal_x: f32,
    left_normal_y: f32,
}

impl Default for PlayerInputState {
    fn default() -> Self {
        PlayerInputState {
            right_normal_x: 0.0,
            right_normal_y: 0.0,
            left_normal_x: 0.0,
            left_normal_y: 0.0,
        }
    }
}

#[system]
#[read_component(PlayerInput)]
#[read_component(Thrusters)]
#[write_component(Orientation)]
#[write_component(Velocity)]
pub fn player_input(
    world: &mut SubWorld,
    #[resource] events: &InputEvents,
    #[resource] time: &DeltaTime,
    #[state] state: &mut PlayerInputState,
) {
    update_orientation(world, events, time, state);
    update_speed(world, events, time, state);
}

// We ignore the possibility of two controllers sending input for now.
fn update_orientation(
    world: &mut SubWorld,
    events: &InputEvents,
    _time: &DeltaTime,
    state: &mut PlayerInputState,
) {
    let normal_x = find_axis_normal(events.iter(), Axis::RightX);
    let normal_y = find_axis_normal(events.iter(), Axis::RightY);

    if let Some(y) = normal_y {
        state.right_normal_y = with_dead_zone(y, 0.05);
    }

    if let Some(x) = normal_x {
        state.right_normal_x = with_dead_zone(x, 0.05);
    }

    if normal_y.is_some() || normal_x.is_some() {
        if let Some(angle) = axis_angle(state.right_normal_x, state.right_normal_y) {
            // Now that we certainly have an angle, we query for the player
            // orientation
            let mut query = <(&mut Orientation, &PlayerInput)>::query();
            query.for_each_mut(world, move |(orientation, _)| {
                orientation.0 = angle;
            });
        }
    }
}

fn update_speed(
    world: &mut SubWorld,
    events: &InputEvents,
    time: &DeltaTime,
    state: &mut PlayerInputState,
) {
    let normal_x = find_axis_normal(events.iter(), Axis::LeftX);
    let normal_y = find_axis_normal(events.iter(), Axis::LeftY);

    if let Some(y) = normal_y {
        state.left_normal_y = y;
    }

    if let Some(x) = normal_x {
        state.left_normal_x = x;
    }

    let dx = with_dead_zone(state.left_normal_x, 0.05);
    let dy = with_dead_zone(state.left_normal_y, 0.05);
    let delta_time = time.as_f32();
    let mut query = <(&mut Velocity, &Thrusters, &PlayerInput)>::query();
    query.for_each_mut(world, move |(velocity, thrusters, _)| {
        velocity.dx = clamp(
            velocity.dx + dx * thrusters.magnitude * delta_time,
            -thrusters.max,
            thrusters.max,
        );
        velocity.dy = clamp(
            velocity.dy + dy * thrusters.magnitude * delta_time,
            -thrusters.max,
            thrusters.max,
        );
    })
}

fn find_axis_normal<'a>(events: impl Iterator<Item = &'a Event>, which_axis: Axis) -> Option<f32> {
    events
        .filter(|event| match event {
            Event::ControllerAxisMotion { axis, .. } if *axis == which_axis => true,
            _ => false,
        })
        .map(|event| match event {
            Event::ControllerAxisMotion { value, .. } => normalize_axis(*value),
            _ => panic!("Event should be guaranteed by filter"),
        })
        .next()
}

/// Convert an axis i16 reading to f32 between -1.0 and 1.0 inclusive
fn normalize_axis(value: i16) -> f32 {
    if value >= 0 {
        value as f32 / 32_767.0
    } else {
        value as f32 / 32_768.0
    }
}

fn with_dead_zone(reading: f32, minimum: f32) -> f32 {
    if reading.abs() > minimum {
        reading
    } else {
        0.0
    }
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    match value {
        x if x < min => min,
        x if x > max => max,
        x => x,
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
