use crate::component::{Bullet, HitMask, Spatial, Sprite, SpriteKind};

pub fn new(spatial: Spatial) -> (Spatial, Sprite, HitMask, Bullet) {
    (
        spatial,
        Sprite {
            kind: SpriteKind::Bullet,
        },
        HitMask::Point,
        Bullet,
    )
}
