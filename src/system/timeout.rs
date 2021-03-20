use std::time::Instant;

use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, IntoQuery};

use crate::component::DestroyTimeout;

#[system]
#[read_component(DestroyTimeout)]
pub fn timeout(world: &mut SubWorld, cmd: &mut CommandBuffer) {
    let now = Instant::now();
    <(Entity, &DestroyTimeout)>::query().for_each_mut(world, |(entity, timeout)| {
        if timeout.when < now {
            cmd.remove(*entity);
        }
    })
}
