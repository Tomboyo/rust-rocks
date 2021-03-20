use std::time::{Duration, Instant};

use crate::component::{Orientation, Position, Sprite, SpriteKind, Timeout, Velocity};

pub fn new(
    p: Position,
    v: Velocity,
    o: Orientation,
) -> (Position, Velocity, Orientation, Sprite, Timeout) {
    (
        p,
        v,
        o,
        Sprite {
            kind: SpriteKind::Bullet,
        },
        Timeout(Instant::now() + Duration::from_secs(2)),
    )
}
