use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, IntoQuery};

use crate::component::{Asteroid, Bullet, HitMask, Position};

#[system]
#[read_component(Entity)]
#[read_component(Asteroid)]
#[read_component(Bullet)]
#[read_component(Position)]
#[read_component(HitMask)]
pub fn collision(world: &mut SubWorld, cmd: &mut CommandBuffer) {
    <(&Position, &HitMask, Entity, &Bullet)>::query().for_each(
        world,
        |(bullet_position, bullet_mask, bullet, _)| {
            <(&Position, &HitMask, Entity, &Asteroid)>::query().for_each(
                world,
                |(asteroid_position, asteroid_mask, asteroid, _)| {
                    if is_collision(
                        bullet_position,
                        bullet_mask,
                        asteroid_position,
                        asteroid_mask,
                    ) {
                        cmd.remove(*bullet);
                        cmd.remove(*asteroid);
                    }
                },
            );
        },
    );
}

fn is_collision(p1: &Position, m1: &HitMask, p2: &Position, m2: &HitMask) -> bool {
    match (m1, m2) {
        (HitMask::Circle { radius }, HitMask::Point) => distance(p1, p2) < *radius,
        (HitMask::Point, HitMask::Circle { radius }) => distance(p1, p2) < *radius,
        _ => panic!("Unsupported collision between {:?} and {:?}", m1, m2),
    }
}

fn distance(p1: &Position, p2: &Position) -> f32 {
    ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt()
}
