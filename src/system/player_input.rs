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
    component::{PlayerInput, Spatial, WrapAround},
    entity,
    resource::{clock::Clock, controllers::Controllers},
};

const MAX_PLAYER_SPEED: f32 = 600.0;
const BULLET_SPEED: f32 = 1_000.0;

struct Input {
    right_x: f32,
    right_y: f32,
    left_x: f32,
    left_y: f32,
}

pub struct SystemState {
    fire_timeout: Instant,
}

pub fn new(clock: &Clock) -> impl Runnable {
    player_input_system(SystemState {
        fire_timeout: clock.now,
    })
}

#[system]
#[read_component(PlayerInput)]
#[write_component(Spatial)]
pub fn player_input(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] controllers: &Arc<Mutex<Controllers>>,
    #[resource] clock: &Clock,
    #[state] state: &mut SystemState,
) {
    let controllers = controllers.lock().unwrap();
    let input = read_input_state(&controllers);

    <(&mut Spatial, &PlayerInput)>::query().for_each_mut(world, |(spatial, _)| {
        if let (_, Some(radians)) = mad(input.right_x, input.right_y) {
            spatial.angle_o = radians;
        }

        // Apply acceleration to the velocity components, then compute the
        // magnitude of the resulting vector. If it is greater than the player's
        // maxspeed, set the vector based on the max speed.
        spatial.dx += input.left_x * MAX_PLAYER_SPEED * clock.delta;
        spatial.dy += input.left_y * MAX_PLAYER_SPEED * clock.delta;
        if let (speed, Some(dir)) = mad(spatial.dx, spatial.dy) {
            if speed > MAX_PLAYER_SPEED {
                spatial.dx = MAX_PLAYER_SPEED * dir.cos();
                spatial.dy = MAX_PLAYER_SPEED * dir.sin();
            }
        }

        if fire_bullet(&controllers) && state.fire_timeout <= clock.now {
            state.fire_timeout = clock.now + Duration::from_millis(200);
            cmd.push(entity::bullet::new(Spatial {
                x: spatial.x,
                y: spatial.y,
                dx: spatial.angle_o.cos() * BULLET_SPEED,
                dy: spatial.angle_o.sin() * BULLET_SPEED,
                angle_o: spatial.angle_o,
                wrap: WrapAround::Destroy,
            }));
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
