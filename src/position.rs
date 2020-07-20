use crate::entity::Entity;

pub fn translate(entity: &mut Entity, modx: f32, mody: f32) {
    entity.x = (entity.x + entity.dx) % modx;
    if entity.x < 0.0 {
        entity.x = modx + entity.x;
    }

    entity.y = (entity.y + entity.dy) % mody;
    if entity.y < 0.0 {
        entity.y = mody + entity.y;
    }
}

pub enum CollisionMask {
    Circle { x: f32, y: f32, radius: f32 },
    Point { x: f32, y: f32 }
}

pub fn collision(a: &CollisionMask, b: &CollisionMask) -> bool {
    match (a, b) {
        (CollisionMask::Circle { x: cx, y: cy, radius },
                CollisionMask::Point { x: px, y: py }) => {
            (px - cx).abs() <= *radius
                && (py - cy) <= *radius
        },
        _ => collision(b, a)
    }
}
