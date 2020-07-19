use crate::entity::Entity;

pub fn translate(entity: &mut Entity, modx: f32, mody: f32) {
    entity.x = (entity.x + entity.dx) % modx;
    entity.y = (entity.y + entity.dy) % mody;
}
