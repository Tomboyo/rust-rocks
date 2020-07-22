use crate::event::Event;
use crate::player;
use crate::position;
use crate::render::Sprite;

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub orientation: f32,
    pub sprite: Sprite,
}

// TODO: remove pub!
pub struct System {
    pub player: Entity,
    pub asteroids: Vec<Entity>,
    pub bullets: Vec<Entity>,
}

// orientatin is in degrees
impl Entity {
    pub fn new(
        x: i32, y: i32,
        dx: f32, dy: f32,
        orientation: f32,
        sprite: Sprite,
    ) -> Entity {
        Entity {
            x: x as f32,
            y: y as f32,
            dx,
            dy,
            orientation,
            sprite,
        }
    }

    pub fn orientation_rad(&self) -> f32 {
        self.orientation * std::f32::consts::PI / 180.0
    }

    pub fn orientation_deg(&self) -> f32 {
        self.orientation
    }
}

impl System {
    pub fn new(
        player: Entity,
        asteroids: Vec<Entity>,
        bullets: Vec<Entity>,
    ) -> System {
        System {
            player,
            asteroids,
            bullets
        }
    }

    // TODO: width/height belong to a position system, not us
    pub fn tick(
        &mut self,
        events: &Vec<Event>,
        render_system: &crate::render::RenderSystem,
        width: f32,
        height: f32,
    ) {
        let mut bullets: Vec<Entity> = events.iter()
            .map(|event| player::handle_event(&mut self.player, event, render_system))
            .filter(|x| matches!(x, Some(_)))
            .map(Option::unwrap)
            .collect();
        self.bullets.append(&mut bullets);

        std::iter::once(&mut self.player)
            .chain(self.asteroids.iter_mut())
            .chain(self.bullets.iter_mut())
            .for_each(|x| position::translate(x, width, height));

        let mut collided_bullets = Vec::new();
        let mut collided_asteroids = Vec::new();
        for (i, bullet) in self.bullets.iter().enumerate() {
            // check for collision with asteroid
            for (j, asteroid) in self.asteroids.iter().enumerate() {
                if position::collision(
                        &position::CollisionMask::Circle {
                            x: asteroid.x,
                            y: asteroid.y,
                            radius: 32.0 // TODO!
                        },
                        &position::CollisionMask::Point {
                            x: bullet.x,
                            y: bullet.y
                        }) {
                    collided_bullets.push(i);
                    collided_asteroids.push(j);
                }
            }
        }

        for b in collided_bullets {
            self.bullets.remove(b);
        }

        for a in collided_asteroids {
            self.asteroids.remove(a);
        }
    }
}
