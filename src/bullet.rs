use std::time::Instant;

use crate::position::Collidable;
use crate::position::Position;
use crate::position::HitMask;
use crate::position::Velocity;
use crate::render::Renderable;
use crate::render::Sprite;

pub struct Bullet {
    pub position: Position,
    pub velocity: Velocity,
    pub orientation: f32, // in degrees
    pub sprite: Sprite,
    pub hitmask: HitMask,
    pub timeout: Timeout,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Timeout {
    Expire { when: Instant },
}

impl Collidable for Bullet {
    fn hit_mask(&self) -> &HitMask {
        &self.hitmask
    }
    
    fn position(&self) -> &Position {
        &self.position
    }
}

impl <'a> Renderable<'a> for Bullet {
    fn position(&'a self) -> &'a Position {
        &self.position
    }

    fn orientation(&self) -> f64 {
        self.orientation as f64
    }

    fn sprite(&'a self) -> &'a Sprite {
        &self.sprite
    }
}
