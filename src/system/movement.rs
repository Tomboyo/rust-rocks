use legion::{system, world::SubWorld, IntoQuery};

use crate::{
    component::{Position, Velocity},
    resource::{bounds::Bounds, delta_time::DeltaTime},
};

#[system]
#[read_component(Velocity)]
#[write_component(Position)]
pub fn movement(world: &mut SubWorld, #[resource] bounds: &Bounds, #[resource] time: &DeltaTime) {
    let mut query = <(&Velocity, &mut Position)>::query();
    query.for_each_mut(world, |(velocity, position)| {
        position.x = (position.x + bounds.width + (velocity.dx * time.as_f32())) % bounds.width;
        position.y = (position.y + bounds.height + (velocity.dy * time.as_f32())) % bounds.height;
    });
}
