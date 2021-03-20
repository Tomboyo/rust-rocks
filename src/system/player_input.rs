use legion::{system, systems::CommandBuffer, world::SubWorld, IntoQuery};
use sdl2::{
    controller::{Axis, Button},
    event::Event,
};

use crate::{
    component::{Orientation, PlayerInput, Position, Thrusters, Velocity},
    entity,
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
#[read_component(Position)]
#[write_component(Orientation)]
#[write_component(Velocity)]
pub fn player_input(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] events: &InputEvents,
    #[resource] time: &DeltaTime,
    #[state] state: &mut PlayerInputState,
) {
    update_state(state, events);

    let delta_time = time.as_f32();
    <(
        &mut Orientation,
        &mut Velocity,
        &Position,
        &Thrusters,
        &PlayerInput,
    )>::query()
    .for_each_mut(world, |(orientation, velocity, position, thrusters, _)| {
        axis_angle(state.right_normal_x, state.right_normal_y).map(|v| orientation.0 = v);

        velocity.dx = clamp(
            velocity.dx + state.left_normal_x * thrusters.magnitude * delta_time,
            -thrusters.max,
            thrusters.max,
        );

        velocity.dy = clamp(
            velocity.dy + state.left_normal_y * thrusters.magnitude * delta_time,
            -thrusters.max,
            thrusters.max,
        );

        if fire_bullet(events) {
            let angle = orientation.0.to_radians();
            cmd.push(entity::bullet::new(
                *position,
                Velocity {
                    dx: angle.cos() * 1_000.0,
                    dy: angle.sin() * 1_000.0,
                },
                *orientation,
            ));
        }
    });
}

fn fire_bullet(events: &InputEvents) -> bool {
    events.iter().any(|event| {
        matches!(
            event,
            Event::ControllerButtonDown {
                button: Button::RightShoulder,
                ..
            }
        )
    })
}

fn update_state(state: &mut PlayerInputState, events: &InputEvents) {
    read_axis(events, Axis::LeftX, 0.3).map(|v| state.left_normal_x = v);
    read_axis(events, Axis::LeftY, 0.3).map(|v| state.left_normal_y = v);
    read_axis(events, Axis::RightX, 0.05).map(|v| state.right_normal_x = v);
    read_axis(events, Axis::RightY, 0.05).map(|v| state.right_normal_y = v);
}

/// Read a controller axis as an f32 in the range 0..=1. All readings are
/// subject to a dead zone such that small readings are changed to 0.0.
fn read_axis<'a>(events: &InputEvents, which_axis: Axis, dead_zone: f32) -> Option<f32> {
    events
        .iter()
        .filter(|event| match event {
            Event::ControllerAxisMotion { axis, .. } if *axis == which_axis => true,
            _ => false,
        })
        .map(|event| match event {
            Event::ControllerAxisMotion { value, .. } => normalize_axis(*value),
            _ => panic!("Event should be guaranteed by filter"),
        })
        .map(|v| if v.abs() > dead_zone { v } else { 0.0 })
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
