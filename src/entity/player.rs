use crate::{
    component::{
        HitMask, Orientation, PlayerInput, Position, Sprite, SpriteKind, Thrusters, Velocity,
    },
    resource::bounds::Bounds,
};

// const MAX_SPEED: f32 = 5.0;

pub fn new(
    bounds: &Bounds,
) -> (
    Position,
    Velocity,
    Thrusters,
    Orientation,
    Sprite,
    HitMask,
    PlayerInput,
) {
    (
        Position {
            x: bounds.width / 2.0,
            y: bounds.height / 2.0,
        },
        Velocity { dx: 0.0, dy: 0.0 },
        Thrusters {
            magnitude: 500.0,
            max: 600.0,
        },
        Orientation(0.0),
        Sprite {
            kind: SpriteKind::Player,
        },
        HitMask::Point,
        PlayerInput,
    )
}
