use std::time::{Duration, Instant};

use crate::component::{
    Bullet, DestroyTimeout, HitMask, Orientation, Position, Sprite, SpriteKind, Velocity,
};

pub fn new(
    p: Position,
    v: Velocity,
    o: Orientation,
) -> (
    Position,
    Velocity,
    Orientation,
    Sprite,
    DestroyTimeout,
    HitMask,
    Bullet,
) {
    (
        p,
        v,
        o,
        Sprite {
            kind: SpriteKind::Bullet,
        },
        DestroyTimeout {
            when: Instant::now() + Duration::from_secs(2),
        },
        HitMask::Point,
        Bullet,
    )
}
