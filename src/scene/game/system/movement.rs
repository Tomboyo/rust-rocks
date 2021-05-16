use legion::{system, systems::CommandBuffer, Entity};

use crate::{
    component::{Spatial, WrapAround},
    resource::{bounds::Bounds, clock::Clock},
};

#[system(for_each)]
#[write_component(Spatial)]
pub fn movement(
    entity: &Entity,
    spatial: &mut Spatial,
    cmd: &mut CommandBuffer,
    #[resource] bounds: &Bounds,
    #[resource] clock: &Clock,
) {
    spatial.x = spatial.x + spatial.dx * clock.dt.as_secs_f32();
    spatial.y = spatial.y + spatial.dy * clock.dt.as_secs_f32();

    match spatial.wrap {
        WrapAround::Destroy => {
            if !bounds.outer.contains(spatial.x, spatial.y) {
                cmd.remove(*entity);
            }
        }
        WrapAround::Wrap => {
            spatial.x = bounds.outer.wrap_x(spatial.x);
            spatial.y = bounds.outer.wrap_y(spatial.y);
        }
    }
}
