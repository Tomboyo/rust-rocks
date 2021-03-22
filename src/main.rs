#![feature(duration_zero)]

mod component;
mod entity;
mod fps_counter;
mod resource;
mod scene;
mod system;

use std::{
    error::Error,
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
};

use fps_counter::FpsCounter;
use resource::{
    bounds::Bounds, controllers::Controllers, input_events::InputEvents, score::Score,
    textures::Textures,
};
use scene::{
    game::GameScene, scene_event::SceneEvent, score::ScoreScene, title::TitleScene, Scene,
};
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

    let ttf = sdl2::ttf::init()?;
    let font = ttf.load_font("resources/press-start-2p/font.ttf", 18)?;

    let video = context.video()?;
    let window = video
        .window("Rust Rocks", bounds.width as u32, bounds.height as u32)
        .position_centered()
        .build()?;
    let canvas = Rc::new(Mutex::new(window.into_canvas().build()?));
    let texture_creator = canvas.lock().unwrap().texture_creator();
    let textures = Rc::new(Textures::new(&texture_creator));

    let mut event_pump = context.event_pump()?;

    let (sender, receiver) = mpsc::channel::<SceneEvent>();
    let sender = Arc::new(Mutex::new(sender));

    let mut scene: Box<dyn Scene> = Box::new(TitleScene::new(
        Rc::clone(&canvas),
        Rc::clone(&textures),
        Arc::clone(&sender),
        &font,
        &texture_creator,
    ));

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(60).unwrap();
    let mut fps_counter = FpsCounter::new();

    let mut high_score = Score::new();

    loop {
        let events: Vec<Event> = event_pump.poll_iter().collect();
        if events.iter().any(|x| matches!(x, Event::Quit { .. })) {
            break;
        }

        let scene_events: Vec<SceneEvent> = receiver.try_iter().collect();
        if scene_events.iter().any(|x| matches!(x, SceneEvent::Quit)) {
            break;
        }

        for scene_event in scene_events {
            match scene_event {
                SceneEvent::GoToGame => {
                    scene = Box::new(GameScene::new(
                        bounds,
                        Rc::clone(&textures),
                        Rc::clone(&canvas),
                        Arc::clone(&sender),
                    ))
                }
                SceneEvent::PlayerHit { current_score } => {
                    scene = Box::new(ScoreScene::new(
                        Rc::clone(&canvas),
                        Arc::clone(&sender),
                        &texture_creator,
                        &font,
                        high_score,
                        current_score,
                    ));

                    if current_score > high_score {
                        high_score = current_score;
                    }
                }
                SceneEvent::GoToTitle => {
                    scene = Box::new(TitleScene::new(
                        Rc::clone(&canvas),
                        Rc::clone(&textures),
                        Arc::clone(&sender),
                        &font,
                        &texture_creator,
                    ))
                }
                _ => (),
            }
        }

        scene.run(InputEvents::new(events));

        if let Some(frames) = fps_counter.tick() {
            log::debug!("FPS = {}", frames);
        }

        fps_manager.delay();
    }

    Ok(())
}
