#![feature(duration_zero)]

mod component;
mod entity;
mod resource;
mod system;

use std::{error::Error, time::Duration};

use entity::{asteroid, player};
use legion::{Resources, Schedule, World};
use resource::{
    bounds::Bounds, controllers::Controllers, delta_time::DeltaTime, input_events::InputEvents,
    textures::Textures,
};
use sdl2::{event::Event, render::Canvas, video::Window, EventPump};
use system::player_input::PlayerInputState;

fn main() {
    env_logger::init();

    let mut world = World::default();
    let bounds = Bounds {
        width: 800.0,
        height: 600.0,
    };
    world.push(asteroid::new(&bounds));
    world.push(asteroid::new(&bounds));
    world.push(asteroid::new(&bounds));
    world.push(asteroid::new(&bounds));
    world.push(asteroid::new(&bounds));
    world.push(player::new(&bounds));

    // game controllers self-close when droppedd; the unused "controllers" holds
    // them open until we are done.
    let (canvas, textures, mut event_pump, _controllers) = create_context().unwrap();
    let mut resources = Resources::default();
    resources.insert(bounds);
    resources.insert(canvas);
    resources.insert(textures);
    resources.insert(InputEvents::new(Vec::new()));
    resources.insert(DeltaTime::new());

    let mut schedule = Schedule::builder()
        .add_thread_local(system::render::render_system())
        .add_system(system::player_input::player_input_system(
            PlayerInputState::default(),
        ))
        .add_system(system::movement_system::movement_system())
        .add_system(system::timeout::timeout_system())
        .build();

    let min = Duration::from_millis(16);
    loop {
        let events: Vec<Event> = event_pump.poll_iter().collect();
        if events.iter().any(|x| matches!(x, Event::Quit { .. })) {
            break;
        }

        resources.remove::<InputEvents>();
        resources.insert(InputEvents::new(events));

        schedule.execute(&mut world, &mut resources);

        let mut delta_time = resources.remove::<DeltaTime>().unwrap();
        if delta_time.elapsed < min {
            std::thread::sleep(min - delta_time.elapsed);
        }

        resources.insert(delta_time.since());
    }
}

fn create_context() -> Result<(Canvas<Window>, Textures, EventPump, Controllers), Box<dyn Error>> {
    let context = sdl2::init()?;
    let gcs = context.game_controller()?;
    let controllers = Controllers::new(&gcs)?;

    let video = context.video()?;
    let window = video
        .window("Rust Rocks", 800, 600)
        .position_centered()
        .build()?;
    let canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();
    let textures = Textures::new(&texture_creator);

    let event_pump = context.event_pump()?;

    Ok((canvas, textures, event_pump, controllers))
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
