use std::time::Instant;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::room::Context;
use crate::room::Room;
use crate::room::RoomTransition;
use crate::scores::Scores;
use crate::scores::Score;

const TEXT_PAD_BOTTOM: u32 = 4;

pub struct ScoreRoom<'r> {
    texture: Texture<'r>
}

impl <'r> ScoreRoom<'r> {
    /// Renders the player's score and high score information for this session.
    /// 
    /// The entire room is rendered to a texture when this function is invoked.
    /// Each render step simply draws that texture to the screen. Any updates to
    /// the score require a new room.
    pub fn new(
        scores: Scores,
        font: &Font,
        texture_creator: &'r TextureCreator<WindowContext>
    ) -> Self {
        let surfaces = Self::render_lines(scores, font);
        let height: u32 = surfaces.iter()
            .map(|s| s.height() + TEXT_PAD_BOTTOM)
            .sum();
        let width = surfaces.iter().map(|s| s.width()).max().unwrap();
        
        let mut composite = Surface::new(
            width,
            height,
            texture_creator.default_pixel_format()
        ).unwrap();

        let mut h: u32 = 0;
        for surface in surfaces {
            surface.blit(
                None,
                &mut composite,
                sdl2::rect::Rect::new(
                    0, // left-adjusted
                    h as i32, // distance from top
                    width,
                    height)
            ).unwrap();
            h += surface.height() + TEXT_PAD_BOTTOM;
        }
        
        Self { texture: composite.as_texture(texture_creator).unwrap() }
    }

    fn render_lines<'f>(
        scores: Scores,
        font: &'f Font
    ) -> Vec<Surface<'f>> {
        let score = scores.score();
        let lines = vec![
            Self::your_score_text(&score),
            Self::high_score_text(&score),
            String::from("Press any key to continue"),
        ];
        
        lines.into_iter()
            .map(|text| Self::text_to_surface(&text, font))
            .collect()
    }

    fn your_score_text(score: &Score) -> String {
        format!("Your score: {}", match score {
            Score::High { score } => score,
            Score::Normal { score, ..} => score,
        })
    }

    fn high_score_text(score: &Score) -> String {
        match score {
            Score::High { .. }
                => String::from("New high score!"),
            Score::Normal { high, .. }
                => format!("High score: {}", high),
        }
    }

    fn text_to_surface<'f>(text: &str, font: &'f Font) -> Surface<'f> {
        font.render(text).solid(Color::WHITE).unwrap()
    }
}

impl <'r> Room for ScoreRoom<'r> {
    fn update(
        &mut self,
        _context: &mut Context,
        events: Vec<Event>,
        _now: Instant
    ) -> Option<RoomTransition> {
        events.iter()
            .find(|event| match event {
                Event::ControllerButtonDown {..} => true,
                Event::KeyDown {..} => true,
                _ => false
            })
            .map(|_| RoomTransition::Title)
    }

    fn render(
        &self,
        context: &mut Context
    ) {
        context.canvas.clear();

        let query = self.texture.query();
        let (window_width, window_height) = context.canvas.window().size();
        
        context.canvas.copy(
            &self.texture,
            None, // render the entire texture
            Rect::new(
                // center the texture horizontally
                ((window_width / 2) - (query.width / 2)) as i32,
                // render 1/3 down from top of screen
                (window_height as f32 * (1.0 / 3.0)) as i32,
                query.width,
                query.height
            )).unwrap();

        context.canvas.present();
    }
}
