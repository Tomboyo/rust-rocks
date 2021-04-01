use crate::{
    component::{HitMask, Player, PlayerInput, Spatial, Sprite, SpriteKind, WrapAround},
    resource::bounds::Bounds,
};

pub fn new(bounds: &Bounds) -> (Spatial, Sprite, HitMask, PlayerInput, Player) {
    (
        Spatial {
            x: bounds.inner.width / 2.0,
            y: bounds.inner.height / 2.0,
            dx: 0.0,
            dy: 0.0,
            angle_o: 0.0,
            wrap: WrapAround::Wrap,
        },
        Sprite {
            kind: SpriteKind::Player,
        },
        HitMask::Point,
        PlayerInput,
        Player,
    )
}
