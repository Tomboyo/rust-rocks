use std::ops::RangeInclusive;

use rand::{prelude::ThreadRng, Rng};

use crate::{
    component::{Asteroid, HitMask, Spatial, Sprite, SpriteKind, WrapAround},
    resource::bounds::Bounds,
};

const SPEED_RANGE: RangeInclusive<f32> = -100.0..=100.0; // pixels per second

/// The "component signature" or "archetype" of an asteroid entity.
pub type Archetype = (Spatial, Sprite, HitMask, Asteroid);

pub fn new<'a>(bounds: &'a Bounds) -> (Spatial, Sprite, HitMask, Asteroid) {
    let mut rng = rand::thread_rng();
    let (x, y) = coords_on_edge(&bounds, &mut rng);
    (
        Spatial {
            x,
            y,
            dx: rng.gen_range(SPEED_RANGE),
            dy: rng.gen_range(SPEED_RANGE),
            angle_o: rng.gen_range(0.0..360.0),
            wrap: WrapAround::Wrap,
        },
        Sprite::default(SpriteKind::Asteroid),
        HitMask::Circle { radius: 32.0 },
        Asteroid,
    )
}

fn coords_on_edge(bounds: &Bounds, rng: &mut ThreadRng) -> (f32, f32) {
    let mut x = bounds.outer.p0.x;
    let mut y = bounds.outer.p0.y;
    if rng.gen::<f32>() < 0.5 {
        y = rng.gen::<f32>() * bounds.outer.height;
        if rng.gen::<f32>() < 0.5 {
            x = bounds.outer.p1.x;
        }
    } else {
        x = rng.gen::<f32>() * bounds.outer.width;
        if rng.gen::<f32>() < 0.5 {
            y = bounds.outer.p1.y;
        }
    }
    (x, y)
}
