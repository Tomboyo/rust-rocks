use std::time::Instant;

use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, IntoQuery};

use crate::component::Timeout;

#[system]
#[read_component(Timeout)]
pub fn timeout(world: &mut SubWorld, cmd: &mut CommandBuffer) {
    let now = Instant::now();
    <(Entity, &Timeout)>::query().for_each_mut(world, |(entity, timeout)| {
        if timeout.0 < now {
            cmd.remove(*entity);
        }
    })
}
