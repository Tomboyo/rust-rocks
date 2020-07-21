use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::event::Event;
use crate::player;
use crate::position;

pub struct Entity<'a> {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub orientation: f32,
    texture: Texture<'a>,
}

// TODO: remove pub!
pub struct System<'a> {
    pub player: Entity<'a>,
    pub asteroids: Vec<Entity<'a>>,
    pub bullets: Vec<Entity<'a>>,
}

// orientatin is in degrees
impl <'a> Entity<'a> {
    pub fn new(
        x: i32, y: i32,
        dx: f32, dy: f32,
        orientation: f32,
        texture: Texture,
    ) -> Entity {
        Entity {
            x: x as f32,
            y: y as f32,
            dx,
            dy,
            orientation,
            texture: texture,
        }
    }

    pub fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.texture.query().width
    }

    pub fn height(&self) -> u32 {
        self.texture.query().height
    }

    pub fn orientation_rad(&self) -> f32 {
        self.orientation * std::f32::consts::PI / 180.0
    }

    pub fn orientation_deg(&self) -> f32 {
        self.orientation
    }
}

impl <'a> System<'a> {
    pub fn new(
        player: Entity<'a>,
        asteroids: Vec<Entity<'a>>,
        bullets: Vec<Entity<'a>>,
    ) -> System<'a> {
        System {
            player,
            asteroids,
            bullets
        }
    }

    // TODO: width/height belong to a position system, not us
    // texture_creator. Figure that out.
    pub fn tick(
        &mut self,
        events: &Vec<Event>,
        texture_creator: &'a TextureCreator<WindowContext>,
        width: f32,
        height: f32,
    ) {
        let mut bullets: Vec<Entity> = events.iter()
            .map(|event| player::handle_event(&mut self.player, event, &texture_creator))
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
