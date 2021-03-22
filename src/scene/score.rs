use std::{
    error::Error,
    rc::Rc,
    sync::{mpsc::Sender, Arc, Mutex},
};

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::{event::Event, render::Canvas};
use sdl2::{pixels::Color, video::Window};

use crate::resource::{input_events::InputEvents, score::Score};

use super::{scene_event::SceneEvent, Scene};

const TEXT_PAD_BOTTOM: u32 = 4;

pub struct ScoreScene {
    canvas: Rc<Mutex<Canvas<Window>>>,
    bus: Arc<Mutex<Sender<SceneEvent>>>,
    texture: Texture,
}

impl ScoreScene {
    pub fn new(
        canvas: Rc<Mutex<Canvas<Window>>>,
        bus: Arc<Mutex<Sender<SceneEvent>>>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
        high_score: Score,
        current_score: Score,
    ) -> Self {
        let texture =
            Self::create_static_texture(&high_score, &current_score, font, texture_creator)
                .unwrap();
        Self {
            canvas,
            bus,
            texture,
        }
    }

    fn create_static_texture(
        high_score: &Score,
        current_score: &Score,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<Texture, Box<dyn Error>> {
        let surfaces = Self::render_lines(high_score, current_score, font);
        let height: u32 = surfaces.iter().map(|s| s.height() + TEXT_PAD_BOTTOM).sum();
        let width = surfaces.iter().map(|s| s.width()).max().unwrap();

        let mut composite =
            Surface::new(width, height, texture_creator.default_pixel_format()).unwrap();

        let mut h: u32 = 0;
        for surface in surfaces {
            surface
                .blit(
                    None,
                    &mut composite,
                    sdl2::rect::Rect::new(
                        0,        // left-adjusted
                        h as i32, // distance from top
                        width, height,
                    ),
                )
                .unwrap();
            h += surface.height() + TEXT_PAD_BOTTOM;
        }

        Ok(composite.as_texture(texture_creator)?)
    }

    fn render_lines<'f>(
        high_score: &Score,
        current_score: &Score,
        font: &'f Font,
    ) -> Vec<Surface<'f>> {
        vec![
            format!("Your score: {}", current_score.as_u32()),
            Self::high_score_text(&high_score, &current_score),
            String::from("Press any key to continue"),
        ]
        .into_iter()
        .map(|text| Self::text_to_surface(&text, font))
        .collect()
    }

    fn high_score_text(high_score: &Score, current_score: &Score) -> String {
        if high_score < current_score {
            format!("New high score!")
        } else {
            format!("High score: {}", high_score.as_u32())
        }
    }

    fn text_to_surface<'f>(text: &str, font: &'f Font) -> Surface<'f> {
        font.render(text).solid(Color::WHITE).unwrap()
    }

    fn handle_input(&mut self, events: &InputEvents) {
        events
            .iter()
            .find(|event| match event {
                Event::ControllerButtonDown { .. } => true,
                Event::KeyDown { .. } => true,
                _ => false,
            })
            .map(|_| {
                self.bus
                    .lock()
                    .unwrap()
                    .send(SceneEvent::GoToTitle)
                    .unwrap()
            });
    }

    fn render(&mut self) {
        let mut canvas = self.canvas.lock().unwrap();
        canvas.clear();

        let query = self.texture.query();
        let (window_width, window_height) = canvas.window().size();

        canvas
            .copy(
                &self.texture,
                None, // render the entire texture
                Rect::new(
                    // center the texture horizontally
                    ((window_width / 2) - (query.width / 2)) as i32,
                    // render 1/3 down from top of screen
                    (window_height as f32 * (1.0 / 3.0)) as i32,
                    query.width,
                    query.height,
                ),
            )
            .unwrap();

        canvas.present();
    }
}

impl Scene for ScoreScene {
    fn run(&mut self, events: InputEvents) {
        self.handle_input(&events);
        self.render();
    }
}
