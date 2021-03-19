#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Thrusters {
    pub magnitude: f32,
    pub max: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Orientation(pub f32);

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
    // Bullet,
    Player,
    // Title,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayerInput;
