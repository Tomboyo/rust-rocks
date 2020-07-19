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
