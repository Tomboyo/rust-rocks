use std::sync::{mpsc::Sender, Arc, Mutex};

use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, IntoQuery};

use crate::{
    component::{Asteroid, Bullet, HitMask, Player, Position},
    resource::score::Score,
    scene::scene_event::SceneEvent,
};

#[system]
#[read_component(Entity)]
#[read_component(Asteroid)]
#[read_component(Player)]
#[read_component(Bullet)]
#[read_component(Position)]
#[read_component(HitMask)]
pub fn collision(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] score: &mut Score,
    #[resource] bus: &mut Arc<Mutex<Sender<SceneEvent>>>,
) {
    let mut asteroids = <(&Position, &HitMask, Entity, &Asteroid)>::query();
    let mut bullets = <(&Position, &HitMask, Entity, &Bullet)>::query();
    let mut players = <(&Position, &HitMask, &Player)>::query();

    asteroids.for_each(world, |asteroid| {
        bullets.for_each(world, |bullet| {
            if is_collision(asteroid.0, asteroid.1, bullet.0, bullet.1) {
                cmd.remove(*asteroid.2);
                cmd.remove(*bullet.2);
                *score += 1;
            }
        });

        let bus = bus.lock().unwrap();
        players.for_each(world, |player| {
            if is_collision(asteroid.0, asteroid.1, player.0, player.1) {
                bus.send(SceneEvent::PlayerHit {
                    current_score: *score,
                })
                .unwrap();
            }
        })
    });
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
