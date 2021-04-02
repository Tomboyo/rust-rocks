use std::{
    rc::Rc,
    sync::{mpsc::Sender, Arc, Mutex},
};

use legion::{Resources, Schedule, World};
use sdl2::{controller::Button, event::Event, render::Canvas, video::Window};

use crate::{
    entity::{asteroid, player},
    resource::{
        bounds::Bounds, clock::Clock, controllers::Controllers, input_events::InputEvents,
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

        let clock = Clock::new();
        let schedule = unpaused_schedule(&clock);

        let mut resources = Resources::default();
        resources.insert(bounds);
        resources.insert(canvas);
        resources.insert(textures);
        resources.insert(controllers);
        resources.insert(bus);
        resources.insert(clock);
        resources.insert(Score::new());

        Self {
            world,
            resources,
            schedule,
        }
    }
}

impl Scene for GameScene {
    fn run(&mut self, events: InputEvents) {
        if events.iter().any(|x| is_pause_event(x)) {
            let mut clock = self.resources.get_mut::<Clock>().unwrap();
            if clock.is_paused {
                clock.unpause();
                self.schedule = unpaused_schedule(&clock);
            } else {
                clock.pause();
                self.schedule = paused_schedule();
            }
        }

        self.resources.insert(events);
        self.schedule.execute(&mut self.world, &mut self.resources);
        self.resources.get_mut::<Clock>().unwrap().advance();
    }
}

fn unpaused_schedule(clock: &Clock) -> Schedule {
    Schedule::builder()
        .add_thread_local(system::render::render_system())
        .add_thread_local(system::player_input::new(clock))
        .add_system(system::movement::movement_system())
        .add_system(system::collision::collision_system())
        .add_system(system::spawn_asteroid::create_spawn_timeout_system())
        .add_system(system::spawn_asteroid::spawn_asteroids_system())
        .build()
}

fn paused_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(system::render::render_system())
        .build()
}

fn is_pause_event(event: &Event) -> bool {
    matches!(
        event,
        Event::ControllerButtonDown {
            button: Button::Start,
            ..
        },
    )
}
