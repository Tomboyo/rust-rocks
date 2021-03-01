use std::ops::Add;
use std::time::Duration;
use std::time::Instant;

use sdl2::controller::Button;
use sdl2::event::Event;

use crate::asteroid::Asteroid;
use crate::bullet::Bullet;
use crate::bullet::Timeout;
use crate::player::Player;
use crate::position;
use crate::position::HitMask;
use crate::position::Position;
use crate::position::Velocity;
use crate::render;
use crate::render::Sprite;
use crate::room::Context;
use crate::room::Room;
use crate::room::RoomTransition;
use crate::{asteroid, position::DynamicPosition};

pub struct GameRoom {
    player: Player,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
    spawn_interval: Duration,
    last_spawn: Instant,
    score: u16,
}

impl GameRoom {
    pub fn new(context: &mut Context) -> Self {
        let (width, height) = context.canvas.window().size();
        Self {
            player: Self::init_player(width, height),
            asteroids: Self::init_asteroids(width, height),
            bullets: Vec::new(),
            spawn_interval: Duration::from_secs(1),
            last_spawn: Instant::now(),
            score: 0,
        }
    }

    fn init_player(width: u32, height: u32) -> Player {
        Player::new((width / 2) as f32, (height / 2) as f32)
    }

    fn init_asteroids(width: u32, height: u32) -> Vec<Asteroid> {
        (1..5).map(|_| asteroid::new(width, height)).collect()
    }
}

impl Room for GameRoom {
    fn update(
        &mut self,
        context: &mut Context,
        events: Vec<Event>,
        now: Instant,
    ) -> Option<RoomTransition> {
        events
            .iter()
            .for_each(|event| self.player.consume_input(context.controllers, event));

        if events.iter().any(Self::is_fire_bullet_event) {
            self.fire_bullet(context);
        }

        self.move_entities(context);
        let tx = self.handle_collisions();
        if tx.is_some() {
            return tx; // player was hit
        }
        self.handle_timeouts();
        self.spawn_asteroids(now, context);

        None
    }

    fn render(&self, context: &mut Context) {
        context.canvas.clear();

        std::iter::once(&self.player as &dyn render::Renderable)
            .chain(self.asteroids.iter().map(|x| x as &dyn render::Renderable))
            .chain(self.bullets.iter().map(|x| x as &dyn render::Renderable))
            .for_each(|x| {
                render::render(context.canvas, context.textures, x)
                    .expect("failed to render bullet")
            });

        context.canvas.present();
    }
}

impl GameRoom {
    fn is_fire_bullet_event(event: &Event) -> bool {
        match event {
            Event::ControllerButtonDown { button, .. } if *button == Button::RightShoulder => true,
            _ => false,
        }
    }

    fn fire_bullet(&mut self, context: &mut Context) {
        let (width, height) = context.textures.dimensions(&self.player.sprite);

        let orientation = self.player.orientation.to_radians();
        let x = orientation.cos() * width as f32 + self.player.position.x;
        let y = orientation.sin() * height as f32 + self.player.position.y;
        let dx = orientation.cos() * 10.0;
        let dy = orientation.sin() * 10.0;

        self.bullets.push(Bullet {
            position: Position { x, y },
            velocity: Velocity { dx, dy },
            orientation: self.player.orientation,
            sprite: Sprite::Bullet,
            hitmask: HitMask::Point,
            timeout: Timeout::Expire {
                when: Instant::now().add(Duration::from_secs(3)),
            },
        });
    }

    fn move_entities(&mut self, context: &Context) {
        let (width, height) = context.canvas.window().size();

        std::iter::once(&mut self.player as &mut dyn DynamicPosition)
            .chain(self.asteroids.iter_mut().map(Self::into_dyn_pos))
            .chain(self.bullets.iter_mut().map(Self::into_dyn_pos))
            .for_each(|x| {
                x.translate();
                x.clamp(width as f32, height as f32);
            });
    }

    fn into_dyn_pos<T: DynamicPosition>(x: &mut T) -> &mut dyn DynamicPosition {
        x
    }

    fn handle_collisions(&mut self) -> Option<RoomTransition> {
        let (_, hits) = position::remove_collisions(&mut self.bullets, &mut self.asteroids);
        self.score += hits as u16;

        if self
            .asteroids
            .iter()
            .any(|x| position::is_collision(&self.player, x))
        {
            Some(RoomTransition::Score { score: self.score })
        } else {
            None
        }
    }

    fn handle_timeouts(&mut self) {
        self.bullets.retain(|x| match x.timeout {
            Timeout::Expire { when } => Instant::now() < when,
        });
    }

    fn spawn_asteroids(&mut self, now: Instant, context: &Context) {
        let (width, height) = context.canvas.window().size();

        if now.duration_since(self.last_spawn) >= self.spawn_interval && self.asteroids.len() < 5 {
            self.last_spawn = now;
            self.asteroids.push(asteroid::new(width, height));
        }
    }
}
