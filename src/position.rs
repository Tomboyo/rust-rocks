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

#[derive(Clone, Debug)]
pub enum HitMask {
    None,
    Circle {
        radius: f32
    },
    Point
}

pub fn collision(a: &Entity, b: &Entity) -> bool {
    match (&a.hitmask, &b.hitmask) {
        (HitMask::Circle { radius }, HitMask::Point) => {
            distance((a.x, a.y), (b.x, b.y)) <= *radius
        },
        (HitMask::Point, HitMask::Circle {..}) => {
            collision(b, a)
        }
        _ => panic!("Collision between {:?} and {:?} not implemented", a.hitmask, b.hitmask)
    }
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((b.0 - a.0).powf(2.0) + (b.1 - a.1).powf(2.0)).sqrt()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::Sprite;

    #[test]
    fn test_collision() {
        let radius = 30.0;
        let circle = Entity {
            x: 100.0, y: 100.0,
            dx: 0.0, dy: 0.0,
            orientation: 0.0,
            sprite: Sprite::Asteroid,
            hitmask: HitMask::Circle { radius }
        };

        let mut point = Entity {
            x: 100.0, y: 100.0,
            dx: 0.0, dy: 0.0,
            orientation: 0.0,
            sprite: Sprite::Bullet,
            hitmask: HitMask::Point
        };

        point.x = circle.x;
        point.y = circle.y;
        assert_eq!(true, collision(&circle, &point),
            "A point within a circle collides");
        
        point.x = circle.x - radius;
        point.y = circle.y;
        assert_eq!(true, collision(&circle, &point),
            "A point on the edge of a circle collides");
        
        point.x = circle.x;
        point.y = circle.y - radius;
        assert_eq!(true, collision(&circle, &point),
            "A point on the edge of a circle collides");

        point.x = circle.x - radius;
        point.y = circle.y - radius;
        assert_eq!(false, collision(&circle, &point),
            "A point outside the circle does not collide");
    }
}
