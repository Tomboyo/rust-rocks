use rand::{prelude::ThreadRng, Rng};

use crate::component::{HitMask, Orientation, Position, Sprite, SpriteKind, Velocity};

static MAX_SPEED: f32 = 7.0;

pub fn new(bounds: (f32, f32)) -> (Position, Velocity, Orientation, Sprite, HitMask) {
    let mut rng = rand::thread_rng();
    let (x, y) = coords_on_edge(bounds, &mut rng);
    (
        Position { x, y },
        Velocity {
            dx: rng.gen_range(-MAX_SPEED..=MAX_SPEED),
            dy: rng.gen_range(-MAX_SPEED..=MAX_SPEED),
        },
        Orientation(rng.gen_range(0.0..360.0)),
        Sprite {
            kind: SpriteKind::Asteroid,
        },
        HitMask::Circle { radius: 10.0 },
    )
}

fn coords_on_edge(bounds: (f32, f32), rng: &mut ThreadRng) -> (f32, f32) {
    let (width, height) = bounds;
    let mut x = 0.0;
    let mut y = 0.0;
    if rng.gen::<f32>() < 0.5 {
        y = rng.gen::<f32>() * height as f32;
        if rng.gen::<f32>() < 0.5 {
            x = width as f32;
        }
    } else {
        x = rng.gen::<f32>() * width as f32;
        if rng.gen::<f32>() < 0.5 {
            y = height as f32;
        }
    }
    (x, y)
}
