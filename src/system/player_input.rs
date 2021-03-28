use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use legion::{
    system,
    systems::{CommandBuffer, Runnable},
    world::SubWorld,
    IntoQuery,
};
use sdl2::controller::{Axis, Button};

use crate::{
    component::{Orientation, PlayerInput, Position, Thrusters, Velocity},
    entity,
    resource::{controllers::Controllers, delta_time::DeltaTime},
};

struct Input {
    right_normal_x: f32,
    right_normal_y: f32,
    left_normal_x: f32,
    left_normal_y: f32,
}

pub struct SystemState {
    fire_timeout: Instant,
}

pub fn new() -> impl Runnable {
    player_input_system(SystemState {
        fire_timeout: Instant::now(),
    })
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
    #[resource] controllers: &Arc<Mutex<Controllers>>,
    #[resource] time: &DeltaTime,
    #[state] state: &mut SystemState,
) {
    let controllers = controllers.lock().unwrap();
    let input = read_input_state(&controllers);

    let delta_time = time.as_f32();
    <(
        &mut Orientation,
        &mut Velocity,
        &Position,
        &Thrusters,
        &PlayerInput,
    )>::query()
    .for_each_mut(world, |(orientation, velocity, position, thrusters, _)| {
        axis_angle(input.right_normal_x, input.right_normal_y).map(|v| orientation.0 = v);

        velocity.dx = clamp(
            velocity.dx + input.left_normal_x * thrusters.magnitude * delta_time,
            -thrusters.max,
            thrusters.max,
        );

        velocity.dy = clamp(
            velocity.dy + input.left_normal_y * thrusters.magnitude * delta_time,
            -thrusters.max,
            thrusters.max,
        );

        if fire_bullet(&controllers) && state.fire_timeout <= Instant::now() {
            state.fire_timeout = Instant::now() + Duration::from_millis(200);
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

fn fire_bullet(controllers: &Controllers) -> bool {
    controllers.vec[0].button(Button::RightShoulder)
}

fn read_input_state(controllers: &Controllers) -> Input {
    Input {
        left_normal_x: read_axis(controllers, Axis::LeftX, 0.05),
        left_normal_y: read_axis(controllers, Axis::LeftY, 0.05),
        right_normal_x: read_axis(controllers, Axis::RightX, 0.05),
        right_normal_y: read_axis(controllers, Axis::RightY, 0.05),
    }
}

/// Read a controller axis as an f32 in the range 0..=1. All readings are
/// subject to a dead zone such that small readings are changed to 0.0.
fn read_axis<'a>(controllers: &Controllers, which_axis: Axis, dead_zone: f32) -> f32 {
    std::iter::once(controllers.vec[0].axis(which_axis))
        .map(|x| normalize_axis(x))
        .map(|v| if v.abs() > dead_zone { v } else { 0.0 })
        .next()
        .unwrap()
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
