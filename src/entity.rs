use std::time::Instant;

use crate::position::HitMask;
use crate::render::Sprite;

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

// orientatin is in degrees
impl Entity {
    pub fn orientation_rad(&self) -> f32 {
        self.orientation * std::f32::consts::PI / 180.0
    }

    pub fn orientation_deg(&self) -> f32 {
        self.orientation
    }
}
