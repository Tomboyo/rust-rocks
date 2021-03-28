use legion::system;

use crate::{
    component::Spatial,
    resource::{bounds::Bounds, delta_time::DeltaTime},
};

#[system(for_each)]
#[write_component(Spatial)]
pub fn movement(spatial: &mut Spatial, #[resource] bounds: &Bounds, #[resource] time: &DeltaTime) {
    spatial.x = (spatial.x + bounds.width + spatial.dx * time.as_f32()) % bounds.width;
    spatial.y = (spatial.y + bounds.height + spatial.dy * time.as_f32()) % bounds.height;
}
