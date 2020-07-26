use std::collections::HashSet;

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

#[derive(Clone, Debug, PartialEq)]
pub enum HitMask {
    None,
    Circle {
        radius: f32
    },
    Point
}

/// For each vector, remove all Entities from it which collide with Entities
/// from the other Vector. Returns the number of entities removed from each
/// vector.
/// 
/// Entities which collide with other Entities in the _same_ vector are ignored.
pub fn remove_collisions(
    a: &mut Vec<Entity>,
    b: &mut Vec<Entity>
) -> (usize, usize) {
    let (collided_a, collided_b) = collisions(a, b);

    remove_all(a, &collided_a);
    remove_all(b, &collided_b);

    (collided_a.len(), collided_b.len())
}

fn remove_all(
    from: &mut Vec<Entity>,
    indices: &HashSet<usize>
) {
    let mut i: usize = 0;
    from.retain(|_| {
        let r = !indices.contains(&i);
        i = i + 1;
        r
    });
}

/// Identifies colliding entities by their indices.
/// 
/// The resulting tuple contains two vectors. The first at position 0 holds
/// indices of elements from `left_source`, while the second at position 1 holds
/// indices of elements from `right_source`.
fn collisions(
    left_source: &Vec<Entity>,
    right_source: &Vec<Entity>
) -> (HashSet<usize>, HashSet<usize>) {
    let mut left_hits = HashSet::new();
    let mut right_hits = HashSet::new();

    for (i, left) in left_source.iter().enumerate() {
        for (j, right) in right_source.iter().enumerate() {
            if is_collision(left, right) {
                left_hits.insert(i);
                right_hits.insert(j);
            }
        }
    }

    (left_hits, right_hits)
}

pub fn is_collision(a: &Entity, b: &Entity) -> bool {
    match (&a.hitmask, &b.hitmask) {
        (HitMask::Circle { radius }, HitMask::Point) => {
            distance((a.x, a.y), (b.x, b.y)) <= *radius
        },
        (HitMask::Point, HitMask::Circle {..}) => {
            is_collision(b, a)
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

    const RADIUS: f32 = 30.0;
    const DIAMETER: f32 = 2.0 * RADIUS;

    fn entity(x: f32, y: f32, hitmask: HitMask) -> Entity {
        Entity {
            x, y,
            dx: 0.0, dy: 0.0,
            orientation: 0.0,
            sprite: Sprite::Asteroid,
            hitmask,
            timeouts: Vec::new(),
        }
    }

    fn circle(x: f32, y: f32) -> Entity {
        entity(x, y, HitMask::Circle { radius: RADIUS })
    }

    fn point(x: f32, y: f32) -> Entity {
        entity(x, y, HitMask::Point)
    }

    #[test]
    fn test_remove_collisions() {
        let mut circles = vec![
            circle(100.0 - DIAMETER, 100.0 - DIAMETER),
            circle(100.0,            100.0           ),
            circle(100.0 + DIAMETER, 100.0 + DIAMETER),
        ];

        let mut points = vec![
            point(100.0 - DIAMETER, 100.0 - DIAMETER),
            point(100.0 - DIAMETER, 100.0 - DIAMETER),
            point(100.0 + DIAMETER, 100.0 + DIAMETER),
            point(100.0 + DIAMETER, 100.0 + DIAMETER),
        ];

        remove_collisions(&mut circles, &mut points);

        assert_eq!(
            vec![circle(100.0, 100.0)],
            circles,
            "The two circles colliding with points should have been removed");
        assert_eq!(
            true,
            points.is_empty(),
            "All points collided with circles and should have been removed");
    }

    #[test]
    fn test_remove_collisions_between_circle_and_points() {
        let x = 100.0;
        let y = 100.0;
        let mut circles = vec![
            circle(x, y)
        ];
        let mut points = vec![
            // In collision (within or on the edge of the circle)
            point(x,          y         ),
            point(x - RADIUS, y         ),
            point(x + RADIUS, y         ),
            point(x,          y - RADIUS),
            point(x,          y + RADIUS),
            // Not in collision (on the corners of a square outside the circle)
            point(x - RADIUS, y - RADIUS),
            point(x - RADIUS, y + RADIUS),
            point(x + RADIUS, y - RADIUS),
            point(x + RADIUS, y + RADIUS),
        ];

        remove_collisions(&mut circles, &mut points);

        assert_eq!(
            true,
            circles.is_empty(),
            "The circle was in collision and should be removed");
        assert_eq!(
            vec![
                point(x - RADIUS, y - RADIUS),
                point(x - RADIUS, y + RADIUS),
                point(x + RADIUS, y - RADIUS),
                point(x + RADIUS, y + RADIUS),
            ],
            points,
            "Only four points did not collide and should remain");
    }
}
