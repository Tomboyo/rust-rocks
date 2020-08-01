use std::ops::Add;
use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::controller::Button;

use crate::asteroid;
use crate::entity::Entity;
use crate::entity::Timeout;
use crate::player::Player;
use crate::position;
use crate::position::IntoCollidable;
use crate::position::Position;
use crate::position::Velocity;
use crate::position::HitMask;
use crate::render;
use crate::render::Sprite;
use crate::room::Context;
use crate::room::Room;
use crate::room::RoomTransition;

pub struct GameRoom {
    player: Player,
    asteroids: Vec<Entity>,
    bullets: Vec<Entity>,
    spawn_interval: Duration,
    last_spawn: Instant,
    score: Score,
}

struct Score {
    pub asteroids_destroyed: u16,
    pub last_death: Instant,
}

impl GameRoom {
    pub fn new(
        context: &mut Context
    ) -> Self {
        let (width, height) = context.canvas.window().size();
        Self {
            player: Self::init_player(width, height),
            asteroids: Self::init_asteroids(width, height),
            bullets: Vec::new(),
            spawn_interval: Duration::from_secs(1),
            last_spawn: Instant::now(),
            score: Score {
                asteroids_destroyed: 0,
                last_death: Instant::now(),
            }
        }
    }

    fn init_player(width: u32, height: u32) -> Player {
        Player::new(
            (width / 2) as f32,
            (height / 2) as f32)
    }

    fn init_asteroids(width: u32, height: u32) -> Vec<Entity> {
        (1..5)
            .map(|_| asteroid::new(width, height))
            .collect()
    }
}


impl Room for GameRoom {
    fn update(
        &mut self,
        context: &mut Context,
        events: Vec<Event>,
        now: Instant
    ) -> Option<RoomTransition> {
        events.iter().for_each(|event| self.player.consume_input(
            context.controllers,
            event));
        
        if events.iter().any(Self::is_fire_bullet_event) {
            self.fire_bullet(context);
        }

        self.move_entities(context);
        self.handle_collisions();
        self.handle_timeouts();
        self.spawn_asteroids(now, context);

        None
    }

    fn render(
        &self,
        context: &mut Context,
    ) {
        context.canvas.clear();

        render::render(
                context.canvas,
                &self.player.position,
                self.player.orientation,
                context.textures.get_texture(&self.player.sprite))
            .expect("failed to render player");
        self.asteroids.iter().for_each(|asteroid| {
            render::render(
                    context.canvas,
                    &asteroid.position,
                    asteroid.orientation,
                    context.textures.get_texture(&asteroid.sprite))
                .expect("failed to render asteroid");
        });
        self.bullets.iter().for_each(|bullet| {
            render::render(
                context.canvas,
                &bullet.position,
                bullet.orientation,
                context.textures.get_texture(&bullet.sprite))
            .expect("failed to render bullet");
        });
        
        context.canvas.present();
    }
}

impl GameRoom {
    fn is_fire_bullet_event(event: &Event) -> bool {
        match event {
            Event::ControllerButtonDown { button, .. }
            if *button == Button::RightShoulder => true,
            _ => false
        }
    }

    fn fire_bullet(
        &mut self,
        context: &mut Context,
    ) {
        let (width, height) = context.textures.dimensions(
            &self.player.sprite);

        let orientation = self.player.orientation.to_radians();
        let x = orientation.cos() * width as f32
            + self.player.position.x;
        let y = orientation.sin() * height as f32
            + self.player.position.y;
        let dx = orientation.cos() * 10.0;
        let dy = orientation.sin() * 10.0;

        self.bullets.push(
            Entity {
                position: Position { x, y },
                velocity: Velocity { dx, dy },
                orientation: self.player.orientation,
                sprite: Sprite::Bullet,
                hitmask: HitMask::Point,
                timeouts: vec![
                    Timeout::Expire {
                        when: Instant::now().add(Duration::from_secs(3)),
                    }
                ],
            });
    }

    fn move_entities(&mut self, context: &Context) {
        let (width, height) = context.canvas.window().size();

        self.player.position = self.player.position.translate(
            &self.player.velocity,
            width as f32,
            height as f32
        );

        self.asteroids.iter_mut()
            .chain(self.bullets.iter_mut())
            .for_each(|entity| {
                entity.position = entity.position.translate(
                    &entity.velocity,
                    width as f32,
                    height as f32);
            });
    }

    fn handle_collisions(&mut self) {
        let (_, hits) = position::remove_collisions(
            &mut self.bullets,
            &mut self.asteroids);
        self.score.asteroids_destroyed += hits as u16;

        if self.score.last_death.elapsed().as_secs() > 5
        && self.asteroids.iter()
            .map(IntoCollidable::into_collidable)
            .any(|x| x.is_collision(&self.player.into_collidable()))
        {
            log::info!(
                "Hit! You destroyed {} asteroids in {} seconds",
                self.score.asteroids_destroyed,
                self.score.last_death.elapsed().as_secs());
            self.score = Score {
                asteroids_destroyed: 0,
                last_death: Instant::now()
            }
        }
    }

    fn handle_timeouts(&mut self) {
        self.bullets.retain(|x| {
            x.timeouts.iter().all(|t| {
                match t {
                    Timeout::Expire { when } => {
                        Instant::now() < *when
                    }
                }
            })
        });
    }

    fn spawn_asteroids(&mut self, now: Instant, context: &Context) {
        let (width, height) = context.canvas.window().size();

        if now.duration_since(self.last_spawn) >= self.spawn_interval
        && self.asteroids.len() < 5 {
            self.last_spawn = now;
            self.asteroids.push(asteroid::new(width, height));
        }
    }
}
