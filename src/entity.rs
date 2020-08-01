use std::time::Instant;

use crate::position::Collidable;
use crate::position::IntoCollidable;
use crate::position::Position;
use crate::position::Velocity;
use crate::position::HitMask;
use crate::render::Sprite;

#[derive(Debug, PartialEq)]
pub struct Entity {
    pub position: Position,
    pub velocity: Velocity,
    pub orientation: f32,
    pub sprite: Sprite,
    pub hitmask: HitMask,
    pub timeouts: Vec::<Timeout>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Timeout {
    Expire { when: Instant },
}

impl IntoCollidable for Entity {
    fn into_collidable(&self) -> Collidable {
        Collidable {
            position: self.position.clone(),
            hitmask: self.hitmask.clone(),
        }
    }
}
