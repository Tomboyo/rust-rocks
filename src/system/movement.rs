use legion::{system, systems::CommandBuffer, Entity};

use crate::{
    component::{Spatial, WrapAround},
    resource::{bounds::Bounds, delta_time::DeltaTime},
};

#[system(for_each)]
#[write_component(Spatial)]
pub fn movement(
    entity: &Entity,
    spatial: &mut Spatial,
    cmd: &mut CommandBuffer,
    #[resource] bounds: &Bounds,
    #[resource] time: &DeltaTime,
) {
    spatial.x = spatial.x + spatial.dx * time.as_f32();
    spatial.y = spatial.y + spatial.dy * time.as_f32();

    match spatial.wrap {
        WrapAround::Destroy => {
            if !bounds.contains(spatial.x, spatial.y) {
                cmd.remove(*entity);
            }
        }
        WrapAround::Wrap => {
            spatial.x = (spatial.x + bounds.width) % bounds.width;
            spatial.y = (spatial.y + bounds.height) % bounds.height;
        }
    }
}
