use std::{
    rc::Rc,
    sync::{mpsc::Sender, Arc, Mutex},
};

use legion::{Resources, Schedule, World};
use sdl2::{render::Canvas, video::Window};

use crate::{
    entity::{asteroid, player},
    resource::{
        bounds::Bounds, controllers::Controllers, delta_time::DeltaTime, input_events::InputEvents,
        score::Score, textures::Textures,
    },
    system,
};

use super::{scene_event::SceneEvent, Scene};

pub struct GameScene {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl GameScene {
    pub fn new(
        bounds: Bounds,
        controllers: Arc<Mutex<Controllers>>,
        textures: Rc<Textures>,
        canvas: Rc<Mutex<Canvas<Window>>>,
        bus: Arc<Mutex<Sender<SceneEvent>>>,
    ) -> Self {
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
        resources.insert(controllers);
        resources.insert(bus);
        resources.insert(DeltaTime::new());
        resources.insert(Score::new());

        let schedule = Schedule::builder()
            .add_thread_local(system::render::render_system())
            .add_thread_local(system::player_input::new())
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
