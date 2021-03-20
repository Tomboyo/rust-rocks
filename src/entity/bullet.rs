use crate::component::{Orientation, Position, Sprite, SpriteKind, Velocity};

pub fn new(p: Position, v: Velocity, o: Orientation) -> (Position, Velocity, Orientation, Sprite) {
    (
        p,
        v,
        o,
        Sprite {
            kind: SpriteKind::Bullet,
        },
    )
}
