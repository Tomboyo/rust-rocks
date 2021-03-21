use std::{rc::Rc, sync::Mutex};

use legion::{Resources, Schedule, World};
use sdl2::{render::Canvas, video::Window};

use crate::{
    entity::{asteroid, player},
    resource::{
        bounds::Bounds, delta_time::DeltaTime, input_events::InputEvents, score::Score,
        textures::Textures,
    },
    system::{self, player_input::PlayerInputState},
};

use super::Scene;

pub struct GameScene {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl GameScene {
    /// Note: acquires a lock on the canvas.
    pub fn new(bounds: Bounds, textures: Rc<Textures>, canvas: Rc<Mutex<Canvas<Window>>>) -> Self {
        let mut world = World::default();

        world.push(asteroid::new(&bounds));
        world.push(asteroid::new(&bounds));
        world.push(asteroid::new(&bounds));
        world.push(asteroid::new(&bounds));
        world.push(asteroid::new(&bounds));
        world.push(player::new(&bounds));

        let mut resources = Resources::default();
        resources.insert(bounds);
        resources.insert(canvas);
        resources.insert(textures);
        resources.insert(DeltaTime::new());
        resources.insert(Score::new());

        let schedule = Schedule::builder()
            .add_thread_local(system::render::render_system())
            .add_system(system::player_input::player_input_system(
                PlayerInputState::default(),
            ))
            .add_system(system::movement::movement_system())
            .add_system(system::timeout::timeout_system())
            .add_system(system::collision::collision_system())
            .add_system(system::spawn_asteroid::create_spawn_timeout_system())
            .add_system(system::spawn_asteroid::spawn_asteroids_system())
            .build();

        Self {
            world,
            resources,
            schedule,
        }
    }
}

impl Scene for GameScene {
    fn run(&mut self, events: InputEvents) {
        self.resources.insert(events);
        self.schedule.execute(&mut self.world, &mut self.resources);
        self.resources.get_mut::<DeltaTime>().unwrap().update();
    }
}
