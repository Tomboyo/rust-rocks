mod asteroid;
mod event;
mod entity;
mod player;
mod position;
mod render;
mod room;

use std::collections::HashMap;
use std::time::Instant;

use log;
use sdl2::GameControllerSubsystem;
use sdl2::controller::GameController;

use room::Context;
use room::Room;
use room::RoomTransition;
use room::game::GameRoom;
use room::title::TitleRoom;

fn main() {
    env_logger::init();

    let context = sdl2::init().expect("Failed to init sdl2 context");
    let gcs = context.game_controller().expect("Failed to init game controller subsystem");
    let mut pump = context.event_pump().expect("Failed to init the event pump");
    let video_subsystem = context.video().expect("Failed to init video subsystem");
    let window = video_subsystem
        .window("Rust Rocks", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");
    let mut canvas = window.into_canvas().build().expect("Failed to create canvas");
    let texture_creator = canvas.texture_creator();
    
    // Controllers self-close when dropped, which stops them from generating
    // events. Hold a vector of controllers until the game loop exits.
    let controllers: HashMap<u32, GameController> = open_controllers(&gcs);
    
    let textures = render::Textures::new(&texture_creator);
    let mut room_context = Context {
        canvas: &mut canvas,
        textures: &textures
    };

    let mut room: Box<dyn Room> = Box::new(TitleRoom::new(&mut room_context));

    loop {
        let events = event::process_events(&mut pump, &controllers);
        if events.iter().any(|x| matches!(x, event::Event::Quit)) {
            break;
        }

        if let Some(transition) = room.update(&mut room_context, events, Instant::now()) {
            match transition {
                RoomTransition::Game => {
                    room = Box::new(GameRoom::new(&mut room_context));
                }
            }
        } else {
            room.render(&mut room_context);
        }

        // TODO: sdl2 has framerate control features
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
