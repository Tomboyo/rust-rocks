#![feature(iter_intersperse)]

mod component;
mod fps_counter;
#[macro_use]
mod controller;
mod font;
mod resource;
mod scene;

use std::{
    error::Error,
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
};

use fps_counter::FpsCounter;
use freetype::Library;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{CloseEvent, EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings};
use resource::{bounds::Bounds, score::Score};
use scene::{GameScene, Scene, SceneEvent, TitleScene};
use sdl2_window::Sdl2Window;

use crate::{font::Font, resource::textures, scene::ScoreScene};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let bounds = Bounds::new(0.0, 0.0, 800.0, 600.0);

    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("Rust Rocks", bounds.inner)
        .exit_on_esc(true)
        .graphics_api(opengl)
        .controllers(true)
        .build()?;
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());

    let (sender, receiver) = mpsc::channel::<SceneEvent>();
    let sender = Arc::new(Mutex::new(sender));

    let freetype = Library::init().unwrap();
    let face = freetype
        .new_face("resources/press-start-2p/font.ttf", 0)
        .unwrap();
    face.set_char_size(18 << 6, 0, 0, 0).unwrap();
    let font = Rc::new(Font::new(&face).unwrap());
    let textures = Rc::new(textures::load_textures());

    let mut scene: Box<dyn Scene> = Box::new(TitleScene::new(
        Arc::clone(&sender),
        Rc::clone(&textures),
        Rc::clone(&font),
    ));

    let mut high_score = Score::new();
    let mut fps = FpsCounter::new();

    // This is a fixed-step event loop. Delta time is constant, but updates-per-
    // second is variable. By default, this will try to update twice per second
    // and render once per second.
    'main: while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.close_args() {
            break;
        }

        if receiver
            .try_iter()
            .any(|event| matches!(event, SceneEvent::Quit))
        {
            break;
        }

        if let Some(args) = e.update_args() {
            scene.update(args);
        } else if let Some(args) = e.render_args() {
            scene.render(args, &mut gl);
            if let Some(frame_rate) = fps.tick() {
                log::debug!("fps: {}", frame_rate);
            }
        } else {
            scene.on_event(e);
        }

        for event in receiver.try_iter() {
            match event {
                SceneEvent::Quit => {
                    break 'main;
                }
                SceneEvent::GoToGame => {
                    scene = Box::new(GameScene::new(
                        Rc::clone(&textures),
                        bounds,
                        Arc::clone(&sender),
                    ))
                }
                SceneEvent::PlayerHit { current_score } => {
                    scene = Box::new(ScoreScene::new(
                        Arc::clone(&sender),
                        Rc::clone(&font),
                        high_score,
                        current_score,
                    ));

                    if current_score > high_score {
                        high_score = current_score;
                    }
                }
                SceneEvent::GoToTitle => {
                    scene = Box::new(TitleScene::new(
                        Arc::clone(&sender),
                        Rc::clone(&textures),
                        Rc::clone(&font),
                    ))
                }
            }
        }
    }

    Ok(())
}

// fn main() -> Result<(), Box<dyn Error>> {
//     env_logger::init();

//     let bounds = Bounds::new(0.0, 0.0, 800.0, 600.0);

//     let context = sdl2::init()?;
//     let gcs = context.game_controller()?;
//     let controllers = Arc::new(Mutex::new(Controllers::new(&gcs)?));

//     let video = context.video()?;
//     let gl_attr = video.gl_attr();
//     gl_attr.set_context_version(3, 3);
//     gl_attr.set_context_profile(GLProfile::Core);
//     gl_attr.set_context_flags().debug().set();
//     // anti-aliasing
//     // gl_attr.set_multisample_buffers(1);
//     // gl_attr.set_multisample_samples(4);

//     let facade = video
//         .window(
//             "Rust Rocks",
//             bounds.inner.width as u32,
//             bounds.inner.height as u32,
//         )
//         .position_centered()
//         .build_glium()?;
//     let facade = Rc::new(Mutex::new(facade));

//     let mut event_pump = context.event_pump()?;

// let (sender, receiver) = mpsc::channel::<SceneEvent>();
// let sender = Arc::new(Mutex::new(sender));

//     let mut scene: Box<dyn Scene> = Box::new(TitleScene::new(
//         Rc::clone(&facade),
//         bounds,
//         Arc::clone(&sender),
//     ));

//     let mut fps_manager = FPSManager::new();
//     fps_manager.set_framerate(60).unwrap();
//     let mut fps_counter = FpsCounter::new();

//     let mut high_score = Score::new();

//     loop {
//         let events: Vec<Event> = event_pump.poll_iter().collect();
//         if events.iter().any(|x| matches!(x, Event::Quit { .. })) {
//             break;
//         };

//         let scene_events: Vec<SceneEvent> = receiver.try_iter().collect();
//         if scene_events.iter().any(|x| matches!(x, SceneEvent::Quit)) {
//             break;
//         }

//         for scene_event in scene_events {
//             match scene_event {
//                 SceneEvent::GoToGame => {
//                     scene = Box::new(GameScene::new(
//                         bounds,
//                         Arc::clone(&controllers),
//                         Rc::clone(&facade),
//                         Arc::clone(&sender),
//                     ));
//                 }
//                 // SceneEvent::PlayerHit { current_score } => {
//                 //     scene = Box::new(ScoreScene::new(
//                 //         Rc::clone(&canvas),
//                 //         Arc::clone(&sender),
//                 //         &texture_creator,
//                 //         &font,
//                 //         high_score,
//                 //         current_score,
//                 //     ));

//                 //     if current_score > high_score {
//                 //         high_score = current_score;
//                 //     }
//                 // }
//                 // SceneEvent::GoToTitle => {
//                 //     scene = Box::new(TitleScene::new(
//                 //         Rc::clone(&canvas),
//                 //         Rc::clone(&textures),
//                 //         Arc::clone(&sender),
//                 //         &font,
//                 //         &texture_creator,
//                 //     ))
//                 // }
//                 _ => (),
//             }
//         }

//         scene.run(InputEvents::new(events));

//         if let Some(frames) = fps_counter.tick() {
//             log::debug!("FPS = {}", frames);
//         }

//         fps_manager.delay();
//     }

//     Ok(())
// }
