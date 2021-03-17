use std::ops::RangeInclusive;

use rand::{prelude::ThreadRng, Rng};

use crate::{
    component::{HitMask, Orientation, Position, Sprite, SpriteKind, Velocity},
    resource::bounds::Bounds,
};

const SPEED_RANGE: RangeInclusive<f32> = -100.0..=100.0; // pixels per second

pub fn new(bounds: &Bounds) -> (Position, Velocity, Orientation, Sprite, HitMask) {
    let mut rng = rand::thread_rng();
    let (x, y) = coords_on_edge(bounds, &mut rng);
    (
        Position { x, y },
        Velocity {
            dx: rng.gen_range(SPEED_RANGE),
            dy: rng.gen_range(SPEED_RANGE),
        },
        Orientation(rng.gen_range(0.0..360.0)),
        Sprite {
            kind: SpriteKind::Asteroid,
        },
        HitMask::Circle { radius: 10.0 },
    )
}

fn coords_on_edge(bounds: &Bounds, rng: &mut ThreadRng) -> (f32, f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    if rng.gen::<f32>() < 0.5 {
        y = rng.gen::<f32>() * bounds.height;
        if rng.gen::<f32>() < 0.5 {
            x = bounds.width;
        }
    } else {
        x = rng.gen::<f32>() * bounds.width;
        if rng.gen::<f32>() < 0.5 {
            y = bounds.height;
        }
    }
    (x, y)
}
