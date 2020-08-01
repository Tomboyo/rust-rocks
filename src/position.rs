use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Position {
    pub fn translate(
        &mut self,
        v: &Velocity,
        modx: f32,
        mody: f32
    ) -> Self {
        let mut x = (self.x + v.dx) % modx;
        if x.is_sign_negative() {
            x = x + modx;
        }

        let mut y = (self.y + v.dy) % mody;
        if y.is_sign_negative() {
            y = y + mody;
        }

        Position { x, y }
    }

    pub fn distance(&self, p: &Self) -> f32 {
        ((self.x - p.x).powf(2.0) + (self.y - p.y).powf(2.0)).sqrt()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HitMask {
    Circle {
        radius: f32
    },
    Point
}

pub trait Collidable {
    fn hit_mask(&self) -> &HitMask;
    fn position(&self) -> &Position;
}

pub fn is_collision(
    a: &impl Collidable,
    b: &impl Collidable
) -> bool {
    match (a.hit_mask(), b.hit_mask()) {
        (HitMask::Circle { radius }, HitMask::Point) => {
            a.position().distance(b.position()) <= *radius
        },
        (HitMask::Point, HitMask::Circle {..}) => {
            is_collision(b, a)
        }
        _ => panic!("Collision between {:?} and {:?} not implemented", a.hit_mask(), b.hit_mask())
    }
}

/// For each vector, remove all Entities from it which collide with Entities
/// from the other Vector. Returns the number of entities removed from each
/// vector.
/// 
/// Entities which collide with other Entities in the _same_ vector are ignored.
pub fn remove_collisions(
    a: &mut Vec<impl Collidable>,
    b: &mut Vec<impl Collidable>
) -> (usize, usize) {
    let (collided_a, collided_b) = collisions(a, b);

    remove_all(a, &collided_a);
    remove_all(b, &collided_b);

    (collided_a.len(), collided_b.len())
}

fn remove_all<T>(
    from: &mut Vec<T>,
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
    left_source: &Vec<impl Collidable>,
    right_source: &Vec<impl Collidable>
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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestCollidable {
        position: Position,
        hit_mask: HitMask,
    }

    impl Collidable for TestCollidable {
        fn position(&self) -> &Position { &self.position }
        fn hit_mask(&self) -> &HitMask { &self.hit_mask }
    }

    const RADIUS: f32 = 30.0;
    const DIAMETER: f32 = 2.0 * RADIUS;

    fn circle(x: f32, y: f32) -> TestCollidable {
        TestCollidable {
            position: Position { x, y },
            hit_mask: HitMask::Circle { radius: RADIUS }
        }
    }

    fn point(x: f32, y: f32) -> TestCollidable {
        TestCollidable {
            position: Position { x, y },
            hit_mask: HitMask::Point
        }
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
