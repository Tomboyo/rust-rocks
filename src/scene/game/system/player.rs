use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

use legion::{system, systems::CommandBuffer, world::SubWorld, IntoQuery};

use crate::{
    component::{PlayerInput, Spatial, WrapAround},
    controller::ControllerState,
    resource::clock::Clock,
    scene::game::entity,
};

const MAX_PLAYER_SPEED: f32 = 600.0;
const BULLET_SPEED: f32 = 1_000.0;

pub struct State {
    pub last_fire_time: Instant,
}

#[system]
#[write_component(PlayerInput)]
#[write_component(Spatial)]
pub fn player(
    world: &mut SubWorld,
    buffer: &mut CommandBuffer,
    #[state] state: &mut State,
    #[resource] clock: &Clock,
    #[resource] controller: &ControllerState,
) {
    // Get the only player.
    let (spatial, _) = <(&mut Spatial, &PlayerInput)>::query()
        .iter_mut(world)
        .next()
        .unwrap();

    if let Some(r) = radians(controller.right_thumb) {
        spatial.angle_o = r as f32;
    }

    // Apply acceleration to the velocity components, then compute the
    // magnitude of the resulting vector. If it is greater than the player's
    // maxspeed, set the vector based on the max speed.
    let (x, y) = controller.left_thumb;
    let dt = clock.dt.as_secs_f32();
    spatial.dx += x as f32 * MAX_PLAYER_SPEED * dt;
    spatial.dy += y as f32 * MAX_PLAYER_SPEED * dt;
    if let Some(r) = radians((spatial.dx as f64, spatial.dy as f64)) {
        let r = r as f32;
        let speed = spatial.dx.hypot(spatial.dy);
        if speed > MAX_PLAYER_SPEED {
            spatial.dx = MAX_PLAYER_SPEED * r.cos();
            spatial.dy = MAX_PLAYER_SPEED * r.sin();
        }
    }

    if controller.right_bumper && clock.now >= state.last_fire_time {
        state.last_fire_time = clock.now + Duration::from_secs_f32(0.333);

        buffer.push(entity::bullet::new(Spatial {
            x: spatial.x,
            y: spatial.y,
            dx: spatial.angle_o.cos() * BULLET_SPEED,
            dy: spatial.angle_o.sin() * BULLET_SPEED,
            angle_o: spatial.angle_o,
            wrap: WrapAround::Destroy,
        }));
    }
}

fn radians(point: (f64, f64)) -> Option<f64> {
    let (x, y) = point;

    if x == 0.0 {
        if y == 0.0 {
            None
        } else if y > 0.0 {
            Some(PI / 2.0)
        } else {
            Some(3.0 * PI / 2.0)
        }
    } else {
        if x > 0.0 {
            // first quadrant: atan positive, magnitude grows towards PI
            // fourth quadrant: atan negative, magnitude decreases towards 2PI
            Some((y / x).atan())
        } else {
            // second quadrant: atan negative, matnitude decreases towards PI
            // thrid quadrant: atan positive, magnitude grows away from PI
            Some(PI + (y / x).atan())
        }
    }
}
