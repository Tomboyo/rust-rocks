mod event;
mod entity;
mod player;
mod position;
mod render;

use std::collections::HashMap;

use log;
use rand::prelude::*;
use sdl2::GameControllerSubsystem;
use sdl2::controller::GameController;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;

use crate::event::Event;

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

    let player_texture = texture_creator.load_texture(
            std::path::Path::new("resources/player-ship.bmp"))
        .expect("Failed to open texture");
    let mut player = entity::Entity::new(400, 300, 0.0, 0.0);
    let mut asteroids: Vec<entity::Entity> =
        (1..5).map(|_| spawn_asteroid()).collect();

    'outer: loop {
        let events = event::process_events(&mut pump, &controllers);

        for event in events.iter() {
            match event {
                Event::Quit => break 'outer,
                _ => player::handle_event(&mut player, event)
            }
        }

        position::translate(&mut player, WIDTH as f32, HEIGHT as f32);
        asteroids.iter_mut()
            .for_each(|x| position::translate(x, WIDTH as f32, HEIGHT as f32));

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        render::render_texture(&mut canvas, &player, &player_texture);
        asteroids.iter_mut()
            .for_each(|x| render::render_asteroid(&mut canvas, x));

        canvas.present();

        // caps fps at 60. Will need an adaptive sleep.
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn spawn_asteroid() -> entity::Entity {
    let mut rng = rand::thread_rng();
    entity::Entity::new(
        (rng.gen::<f32>() * WIDTH as f32) as i32,
        (rng.gen::<f32>() * WIDTH as f32) as i32,
        rng.gen::<f32>() * 5.0,
        rng.gen::<f32>() * 5.0,
    )
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
