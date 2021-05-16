use std::time::Instant;

mod spatial;
mod sprite;

pub use spatial::Spatial;
pub use sprite::Sprite;
pub use sprite::SpriteKind;

pub struct Asteroid;

pub struct Bullet;

pub struct Player;

#[derive(Copy, Clone)]
pub enum WrapAround {
    /// An entity should wrap around the edge of the screen
    Wrap,
    /// An entity should be removed from play after exiting the screen
    Destroy,
}

pub struct SpawnTimeout {
    pub when: Instant,
}

#[derive(Clone, Copy, Debug)]
pub enum HitMask {
    Circle { radius: f32 },
    Point,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayerInput;
