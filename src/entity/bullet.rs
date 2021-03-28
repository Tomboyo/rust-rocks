use std::time::{Duration, Instant};

use crate::component::{Bullet, DestroyTimeout, HitMask, Spatial, Sprite, SpriteKind};

pub fn new(spatial: Spatial) -> (Spatial, Sprite, DestroyTimeout, HitMask, Bullet) {
    (
        spatial,
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
