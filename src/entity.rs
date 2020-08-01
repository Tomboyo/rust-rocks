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

// orientatin is in degrees
impl Entity {
    pub fn orientation_rad(&self) -> f32 {
        self.orientation * std::f32::consts::PI / 180.0
    }

    pub fn orientation_deg(&self) -> f32 {
        self.orientation
    }
}

impl IntoCollidable for Entity {
    fn into_collidable(&self) -> Collidable {
        Collidable {
            position: self.position.clone(),
            hitmask: self.hitmask.clone(),
        }
    }
}
