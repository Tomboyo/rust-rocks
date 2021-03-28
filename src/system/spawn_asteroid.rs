use std::time::{Duration, Instant};

use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, IntoQuery};

use crate::{
    component::{Asteroid, SpawnTimeout},
    entity::{self, asteroid::Archetype},
    resource::bounds::Bounds,
};

#[system]
#[read_component(SpawnTimeout)]
#[read_component(Asteroid)]
pub fn create_spawn_timeout(world: &mut SubWorld, cmd: &mut CommandBuffer) {
    let asteroids =
        <&Asteroid>::query().iter(world).count() + <&SpawnTimeout>::query().iter(world).count();
    if asteroids < 5 {
        let when = Instant::now() + Duration::from_secs(1);
        let timeouts: Vec<(SpawnTimeout,)> = (0..(5 - asteroids))
            .map(|_| (SpawnTimeout { when },))
            .collect();
        cmd.extend(timeouts);
    }
}

#[system]
#[read_component(SpawnTimeout)]
pub fn spawn_asteroids(world: &mut SubWorld, cmd: &mut CommandBuffer, #[resource] bounds: &Bounds) {
    let now = Instant::now();
    let asteroids: Vec<Archetype> = <(&SpawnTimeout, Entity)>::query()
        .iter(world)
        .filter(|(timeout, _entity)| timeout.when < now)
        .map(|(_timeout, entity)| {
            cmd.remove(*entity);
        })
        .map(|_| entity::asteroid::new(bounds))
        .collect();
    cmd.extend(asteroids);
}
