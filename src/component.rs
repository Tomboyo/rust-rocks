use std::time::Instant;

pub struct Asteroid;

pub struct Bullet;

pub struct Player;

pub struct Spatial {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    /// angle of orientation ("where it's pointing"), in radians
    pub angle_o: f32,
}

pub struct DestroyTimeout {
    pub when: Instant,
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
pub struct Sprite {
    pub kind: SpriteKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpriteKind {
    Asteroid,
    Bullet,
    Player,
    Title,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayerInput;
