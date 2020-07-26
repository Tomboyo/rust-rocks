use std::time::Duration;
use std::time::Instant;

use crate::asteroid;
use crate::event::Event;
use crate::entity::Entity;
use crate::entity::Timeout;
use crate::player;
use crate::position;
use crate::render;
use crate::room::Context;
use crate::room::Room;
use crate::room::RoomTransition;

pub struct GameRoom {
    player: Entity,
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
            player: init_player(width, height),
            asteroids: init_asteroids(width, height),
            bullets: Vec::new(),
            spawn_interval: Duration::from_secs(1),
            last_spawn: Instant::now(),
            score: Score {
                asteroids_destroyed: 0,
                last_death: Instant::now(),
            }
        }
    }

    fn render(
        &mut self,
        context: &mut Context,
    ) {
        render::render(
            context.canvas,
            context.textures,
            std::iter::once(&self.player)
                .chain(self.asteroids.iter())
                .chain(self.bullets.iter()));
    }
}

fn init_player(width: u32, height: u32) -> Entity {
    player::new(
        (width / 2) as f32,
        (height / 2) as f32)
}

fn init_asteroids(width: u32, height: u32) -> Vec<Entity> {
    (1..5)
        .map(|_| asteroid::new(width, height))
        .collect()
}

impl Room for GameRoom {
    fn run(
        &mut self,
        context: &mut Context,
        events: Vec<Event>,
        now: Instant
    ) -> Option<RoomTransition> {
        let mut bullets: Vec<Entity> = events.iter()
            .map(|event| player::handle_event(
                &mut self.player,
                event,
                context.textures))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        self.bullets.append(&mut bullets);

        let (width, height) = context.canvas.window().size();
        std::iter::once(&mut self.player)
            .chain(self.asteroids.iter_mut())
            .chain(self.bullets.iter_mut())
            .for_each(|x| position::translate(
                x, width as f32, height as f32));

        let (_, hits) = position::remove_collisions(
            &mut self.bullets,
            &mut self.asteroids);
        self.score.asteroids_destroyed += hits as u16;

        if self.score.last_death.elapsed().as_secs() > 5
        && self.asteroids.iter()
            .any(|x| position::is_collision(&self.player, x))
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

        self.bullets.retain(|x| {
            x.timeouts.iter().all(|t| {
                match t {
                    Timeout::Expire { when } => {
                        Instant::now() < *when
                    }
                }
            })
        });

        if now.duration_since(self.last_spawn) >= self.spawn_interval
            && self.asteroids.len() < 5 {
            self.last_spawn = now;
            self.asteroids.push(asteroid::new(width, height));
        }

        self.render(context);

        None
    }
}
