use std::time::Duration;
use std::time::Instant;

use crate::event::Event;
use crate::player;
use crate::position;
use crate::position::HitMask;
use crate::render::Sprite;
use crate::render::Textures;

#[derive(Debug, PartialEq)]
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub orientation: f32,
    pub sprite: Sprite,
    pub hitmask: HitMask,
    pub timeouts: Vec::<Timeout>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Timeout {
    Expire { when: Instant },
}

pub struct System {
    pub player: Entity,
    pub asteroids: Vec<Entity>,
    pub bullets: Vec<Entity>,
    pub spawn_interval: Duration,
    last_spawn: Instant
}

// orientatin is in degrees
impl Entity {
    pub fn orientation_rad(&self) -> f32 {
        self.orientation * std::f32::consts::PI / 180.0
    }

    pub fn orientation_deg(&self) -> f32 {
        self.orientation
    }
}

impl System {
    pub fn new(
        player: Entity,
        asteroids: Vec<Entity>,
        bullets: Vec<Entity>,
        spawn_interval: Duration
    ) -> System {
        System {
            player,
            asteroids,
            bullets,
            spawn_interval,
            last_spawn: Instant::now()
        }
    }

    // TODO: width/height belong to a position system, not us
    pub fn tick(
        &mut self,
        events: &Vec<Event>,
        textures: &Textures,
        width: f32,
        height: f32,
        now: Instant
    ) {
        let mut bullets: Vec<Entity> = events.iter()
            .map(|event| player::handle_event(&mut self.player, event, textures))
            .filter(|x| matches!(x, Some(_)))
            .map(Option::unwrap)
            .collect();
        self.bullets.append(&mut bullets);

        std::iter::once(&mut self.player)
            .chain(self.asteroids.iter_mut())
            .chain(self.bullets.iter_mut())
            .for_each(|x| position::translate(x, width, height));

        position::remove_collisions(&mut self.bullets, &mut self.asteroids);

        self.bullets.retain(|x| {
            x.timeouts.iter().all(|t| {
                match t {
                    Timeout::Expire { when } => {
                        Instant::now() < *when
                    }
                }
            })
        });

        if now.duration_since(self.last_spawn) >= self.spawn_interval
            && self.asteroids.len() < 5 {
            self.last_spawn = now;
            self.asteroids.push(crate::asteroid::new((width as u32, height as u32)));
        }
    }
}
