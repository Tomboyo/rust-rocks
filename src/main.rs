#![feature(duration_zero)]

mod component;
mod entity;
mod fps_counter;
mod resource;
mod scene;
mod system;

use std::{error::Error, rc::Rc, sync::Mutex};

use fps_counter::FpsCounter;
use resource::{
    bounds::Bounds, controllers::Controllers, input_events::InputEvents, textures::Textures,
};
use scene::{game::GameScene, Scene};
use sdl2::{event::Event, gfx::framerate::FPSManager};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let bounds = Bounds {
        width: 800.0,
        height: 600.0,
    };

    let context = sdl2::init()?;
    let gcs = context.game_controller()?;
    let _controllers = Controllers::new(&gcs)?;

    let video = context.video()?;
    let window = video
        .window("Rust Rocks", bounds.width as u32, bounds.height as u32)
        .position_centered()
        .build()?;
    let canvas = Rc::new(Mutex::new(window.into_canvas().build()?));
    let texture_creator = canvas.lock().unwrap().texture_creator();
    let textures = Rc::new(Textures::new(&texture_creator));

    let mut event_pump = context.event_pump()?;

    let mut scene = GameScene::new(bounds, Rc::clone(&textures), Rc::clone(&canvas));

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(60).unwrap();
    let mut fps_counter = FpsCounter::new();

    loop {
        let events: Vec<Event> = event_pump.poll_iter().collect();
        if events.iter().any(|x| matches!(x, Event::Quit { .. })) {
            break;
        }

        scene.run(InputEvents::new(events));

        if let Some(frames) = fps_counter.tick() {
            log::debug!("FPS = {}", frames);
        }

        fps_manager.delay();
    }

    Ok(())
}

// fn legacy_main() {
//     env_logger::init();

//     let context = sdl2::init().expect("Failed to init sdl2 context");
//     let gcs = context
//         .game_controller()
//         .expect("Failed to init game controller subsystem");
//     let mut pump = context.event_pump().expect("Failed to init the event pump");
//     let video_subsystem = context.video().expect("Failed to init video subsystem");
//     let ttf = sdl2::ttf::init().expect("Failed to init TTF subsystem");
//     let font = ttf
//         .load_font("resources/press-start-2p/font.ttf", 18)
//         .expect("Failed to load press-start-2p font");
//     let window = video_subsystem
//         .window("Rust Rocks", 800, 600)
//         .position_centered()
//         .build()
//         .expect("Failed to create window");
//     let mut canvas = window
//         .into_canvas()
//         .build()
//         .expect("Failed to create canvas");
//     let texture_creator = canvas.texture_creator();

//     // Note: Controllers self-close when dropped, which stops them from
//     // generating events.
//     let controllers = ControllersMap::open(&gcs);

//     let textures = render::Textures::new(&texture_creator);
//     let mut room_context = Context {
//         canvas: &mut canvas,
//         textures: &textures,
//         controllers: &controllers,
//     };

//     let mut scores = Scores::init();
//     let mut room: Box<dyn Room> = Box::new(TitleRoom::new(&font, &texture_creator));

//     loop {
//         let events: Vec<Event> = pump.poll_iter().collect();
//         if events.iter().any(|x| matches!(x, Event::Quit { .. })) {
//             break;
//         }

//         if let Some(transition) = room.update(&mut room_context, events, Instant::now()) {
//             match transition {
//                 RoomTransition::Game => {
//                     room = Box::new(GameRoom::new(&mut room_context));
//                 }
//                 RoomTransition::Title => room = Box::new(TitleRoom::new(&font, &texture_creator)),
//                 RoomTransition::Score { score } => {
//                     scores.new_score(score);
//                     room = Box::new(ScoreRoom::new(scores.clone(), &font, &texture_creator))
//                 }
//                 RoomTransition::Quit => break,
//             }
//         } else {
//             room.render(&mut room_context);
//         }

//         // TODO: sdl2 has framerate control features
//         std::thread::sleep(std::time::Duration::from_millis(16));
//     }
// }
