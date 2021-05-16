mod entity;
mod system;

use std::{
    rc::Rc,
    sync::{mpsc::Sender, Arc, Mutex},
    time::Duration,
};

use legion::{Resources, Schedule, World};
use opengl_graphics::GlGraphics;
use piston::{Event, RenderArgs, UpdateArgs};

use crate::{
    component::{Spatial, Sprite},
    controller::ControllerState,
    resource::{bounds::Bounds, clock::Clock, score::Score, textures::Textures},
};

use super::{scene_event::SceneEvent, Scene};

pub struct GameScene {
    textures: Rc<Textures>,
    world: World,
    resources: Resources,
    update: Schedule,
    is_paused: bool,
    clock: Clock,
    controller: ControllerState,
}

impl GameScene {
    pub fn new(
        textures: Rc<Textures>,
        bounds: Bounds,
        bus: Arc<Mutex<Sender<SceneEvent>>>,
    ) -> Self {
        let mut world = World::default();

        world.push(entity::asteroid::new(&bounds));
        world.push(entity::asteroid::new(&bounds));
        world.push(entity::asteroid::new(&bounds));
        world.push(entity::asteroid::new(&bounds));
        world.push(entity::asteroid::new(&bounds));
        world.push(entity::player::new(&bounds));

        let mut resources = Resources::default();
        resources.insert(bounds);
        resources.insert(bus);
        resources.insert(Score::new());

        let clock = Clock::new();

        Self {
            textures,
            world,
            resources,
            update: update_schedule(&clock),
            is_paused: false,
            clock,
            controller: ControllerState::new(),
        }
    }

    fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }
}

impl Scene for GameScene {
    fn update(&mut self, args: UpdateArgs) {
        if self.is_paused {
            return;
        }

        self.clock.update(Duration::from_secs_f64(args.dt));
        self.resources.insert(self.clock);
        self.resources.insert(self.controller);

        self.update.execute(&mut self.world, &mut self.resources);
    }

    fn render(&mut self, args: RenderArgs, gl: &mut GlGraphics) {
        use graphics::{clear, image, Transformed};
        use legion::IntoQuery;

        gl.draw(args.viewport(), |c, g| {
            clear([0.0; 4], g);
            <(&Spatial, &Sprite)>::query().for_each(&self.world, |(spatial, sprite)| {
                let texture = self.textures.get(&sprite.kind).unwrap();

                let transform = c
                    .trans(spatial.x as f64, spatial.y as f64)
                    .rot_rad(spatial.angle_o as f64)
                    .trans(sprite.width as f64 / -2.0, sprite.height as f64 / -2.0)
                    .transform;

                image(texture, transform, g)
            });
        })
    }

    fn on_event(&mut self, event: Event) {
        match event {
            // start
            button_press!(button) if button == 7 => self.toggle_pause(),
            // right bumper
            button_press!(button) if button == 5 => self.controller.press_right_bumper(),
            button_release!(button) if button == 5 => self.controller.release_right_bumper(),
            // thumbsticks
            axis!(axis, position) => {
                self.controller.update_axis(axis, position);
            }
            _ => (),
        }
    }
}

fn update_schedule(clock: &Clock) -> Schedule {
    Schedule::builder()
        .add_system(system::player::player_system(system::player::State {
            last_fire_time: clock.now,
        }))
        .add_system(system::movement::movement_system())
        .add_system(system::collision::collision_system())
        .add_system(system::spawn_asteroid::create_spawn_timeout_system())
        .add_system(system::spawn_asteroid::spawn_asteroids_system())
        .build()
}
