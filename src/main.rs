mod asteroid;
mod bullet;
mod input;
mod player;
mod position;
mod render;
mod room;

use std::time::Instant;

use sdl2::event::Event;

use crate::input::ControllersMap;
use crate::room::Context;
use crate::room::Room;
use crate::room::RoomTransition;
use crate::room::game::GameRoom;
use crate::room::title::TitleRoom;

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
    
    // Note: Controllers self-close when dropped, which stops them from
    // generating events.
    let controllers = ControllersMap::open(&gcs);
    
    let textures = render::Textures::new(&texture_creator);
    let mut room_context = Context {
        canvas: &mut canvas,
        textures: &textures,
        controllers: &controllers,
    };

    let mut room: Box<dyn Room> = Box::new(TitleRoom::new(&mut room_context));

    loop {
        let events: Vec<Event> = pump.poll_iter().collect();
        if events.iter().any(|x| matches!(x, Event::Quit {..})) {
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
