mod asteroid;
mod event;
mod entity;
mod player;
mod position;
mod render;

use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use log;
use sdl2::GameControllerSubsystem;
use sdl2::controller::GameController;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 600;

fn main() {
    env_logger::init();

    let context = sdl2::init().expect("Failed to init sdl2 context");
    let gcs = context.game_controller().expect("Failed to init game controller subsystem");
    let mut pump = context.event_pump().expect("Failed to init the event pump");
    let video_subsystem = context.video().expect("Failed to init video subsystem");
    let window = video_subsystem
        .window("Asteroids-Rs", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to create window");
    let mut canvas = window.into_canvas().build().expect("Failed to create canvas");
    let texture_creator = canvas.texture_creator();

    // Controllers self-close when dropped, which stops them from generating
    // events. Hold a vector of controllers until the game loop exits.
    let controllers: HashMap<u32, GameController> = open_controllers(&gcs);

    let mut entity_system = entity::System::new(
        player::new(
            (400, 300),
            (0.0, 0.0)),
        (1..5)
            .map(|_| asteroid::new((WIDTH, HEIGHT)))
            .collect(),
        Vec::new(),
        Duration::from_secs(1));
    let textures = render::Textures::new(&texture_creator);

    loop {
        let events = event::process_events(&mut pump, &controllers);
        if events.iter().any(|x| matches!(x, event::Event::Quit)) {
            break;
        }

        entity_system.tick(
            &events,
            &textures,
            WIDTH as f32,
            HEIGHT as f32,
            Instant::now());

        render::render(
            &mut canvas,
            &textures,
            std::iter::once(&entity_system.player)
                .chain(entity_system.asteroids.iter())
                .chain(entity_system.bullets.iter()));

        // caps fps at 60. Will need an adaptive sleep.
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn open_controllers(gcs: &GameControllerSubsystem) -> HashMap<u32, GameController> {
    (0..gcs.num_joysticks().expect("Unable to iterate joysticks"))
        .map(|index| (index, gcs.open(index)))
        .inspect(|(_, result)| match result {
            Ok(controller) => log::info!("Opened controller {}", controller.name()),
            Err(e) => log::warn!("Failed to open controller: {}", e)
        })
        .filter(|(_, result)| result.is_ok())
        .map(|(index, result)| (index, result.unwrap()))
        .collect()
}
