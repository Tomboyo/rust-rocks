use crate::{
    component::{HitMask, Player, PlayerInput, Spatial, Sprite, SpriteKind},
    resource::bounds::Bounds,
};

pub fn new(bounds: &Bounds) -> (Spatial, Sprite, HitMask, PlayerInput, Player) {
    (
        Spatial {
            x: bounds.width / 2.0,
            y: bounds.height / 2.0,
            dx: 0.0,
            dy: 0.0,
            angle_o: 0.0,
        },
        Sprite {
            kind: SpriteKind::Player,
        },
        HitMask::Point,
        PlayerInput,
        Player,
    )
}
