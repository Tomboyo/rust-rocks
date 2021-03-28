use std::{
    f32::consts::PI,
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
    right_x: f32,
    right_y: f32,
    left_x: f32,
    left_y: f32,
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
        if let (_, Some(radians)) = mad(input.right_x, input.right_y) {
            orientation.0 = radians.to_degrees();
        }

        // Apply acceleration to the velocity components, then compute the
        // magnitude of the resulting vector. If it is greater than the player's
        // maxspeed, set the vector based on the max speed.
        velocity.dx += input.left_x * thrusters.magnitude * delta_time;
        velocity.dy += input.left_y * thrusters.magnitude * delta_time;
        if let (speed, Some(dir)) = mad(velocity.dx, velocity.dy) {
            if speed > thrusters.max {
                velocity.dx = thrusters.max * dir.cos();
                velocity.dy = thrusters.max * dir.sin();
            }
        }

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
    let mut input = Input {
        left_x: read_axis(controllers, Axis::LeftX),
        left_y: read_axis(controllers, Axis::LeftY),
        right_x: read_axis(controllers, Axis::RightX),
        right_y: read_axis(controllers, Axis::RightY),
    };

    // Apply a dead zone: read zero for any reading within a small distance from
    // the neutral position.

    if input.left_x.hypot(input.left_y) < 0.05 {
        input.left_x = 0.0;
        input.left_y = 0.0;
    }

    if input.right_x.hypot(input.right_y) < 0.05 {
        input.right_x = 0.0;
        input.right_y = 0.0;
    }

    input
}

/// Read a controller axis as an f32 in the range 0..=1.
fn read_axis<'a>(controllers: &Controllers, which_axis: Axis) -> f32 {
    normalize_axis(controllers.vec[0].axis(which_axis))
}

/// Convert an axis i16 reading to f32 between -1.0 and 1.0 inclusive
fn normalize_axis(value: i16) -> f32 {
    if value >= 0 {
        value as f32 / 32_767.0
    } else {
        value as f32 / 32_768.0
    }
}

/// Get the magnitude and direction in radians of the vector from the origin to
/// the given point. If the point is the origin, this returns (0.0, None). None
/// is never returned otherwise.
fn mad(x: f32, y: f32) -> (f32, Option<f32>) {
    if x == 0.0 {
        if y == 0.0 {
            (0.0, None)
        } else if y > 0.0 {
            (y.abs(), Some(PI / 2.0))
        } else {
            (y.abs(), Some(3.0 * PI / 2.0))
        }
    } else {
        let hypotenuse = x.hypot(y);
        let angle = if x > 0.0 {
            // first quadrant: atan positive, magnitude grows towards PI
            // fourth quadrant: atan negative, magnitude decreases towards 2PI
            (y / x).atan()
        } else {
            // second quadrant: atan negative, matnitude decreases towards PI
            // thrid quadrant: atan positive, magnitude grows away from PI
            PI + (y / x).atan()
        };

        (hypotenuse, Some(angle))
    }
}
