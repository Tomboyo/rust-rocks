mod component;
mod entity;
mod resource;
mod system;

use std::{
    error::Error,
    time::{Duration, Instant},
};

use entity::asteroid;
use legion::{Resources, Schedule, World};
use resource::textures::Textures;
use sdl2::{render::Canvas, video::Window};

fn main() {
    env_logger::init();

    let mut world = World::default();
    world.push(asteroid::new((800.0, 600.0)));

    let mut resources = Resources::default();
    let (canvas, textures) = create_context().unwrap();
    resources.insert(canvas);
    resources.insert(textures);

    let mut schedule = Schedule::builder()
        .add_thread_local(system::render::render_system())
        .build();

    let min = Duration::from_millis(16);
    loop {
        let start = Instant::now();

        schedule.execute(&mut world, &mut resources);

        let elapsed = start.elapsed();
        if elapsed < min {
            std::thread::sleep(min - elapsed);
        }
    }
}

fn create_context() -> Result<(Canvas<Window>, Textures), Box<dyn Error>> {
    let context = sdl2::init()?;
    let video = context.video()?;
    let window = video
        .window("Rust Rocks", 800, 600)
        .position_centered()
        .build()?;
    let canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();
    let textures = Textures::new(&texture_creator);
    Ok((canvas, textures))
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
