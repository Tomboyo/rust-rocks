use std::{
    rc::Rc,
    sync::{mpsc::Sender, Arc, Mutex},
};

use graphics::color;
use opengl_graphics::GlGraphics;
use piston::{Event, RenderArgs};

use crate::{font::Font, resource::score::Score};

use super::{scene_event::SceneEvent, Scene};

pub struct ScoreScene {
    bus: Arc<Mutex<Sender<SceneEvent>>>,
    font: Rc<Font>,
    current_score: Score,
    high_score: Score,
}

impl ScoreScene {
    pub fn new(
        bus: Arc<Mutex<Sender<SceneEvent>>>,
        font: Rc<Font>,
        high_score: Score,
        current_score: Score,
    ) -> Self {
        Self {
            bus,
            font,
            current_score,
            high_score,
        }
    }
}

impl Scene for ScoreScene {
    fn update(&mut self, _args: piston::UpdateArgs) {}

    fn render(&mut self, args: RenderArgs, gl: &mut GlGraphics) {
        let lines = if self.current_score > self.high_score {
            vec![
                self.font
                    .create_text(&format!("New high score: {}", self.current_score))
                    .unwrap(),
                self.font.create_text("Press any key to continue.").unwrap(),
            ]
        } else {
            vec![
                self.font
                    .create_text(&format!("Your score: {}", self.current_score))
                    .unwrap(),
                self.font
                    .create_text(&format!("High score: {}", self.high_score))
                    .unwrap(),
            ]
        };

        gl.draw(args.viewport(), |c, g| {
            use graphics::clear;

            clear([0.0; 4], g);

            let center = {
                let [_, _, width, height] = args.viewport().rect;
                (width as f64 / 2.0, height as f64 / 2.0)
            };

            self.font
                .render_lines(
                    &lines,
                    crate::font::Layout::LeftAlign,
                    center,
                    color::WHITE,
                    &c,
                    g,
                )
                .unwrap();
        })
    }

    fn on_event(&mut self, event: Event) {
        match event {
            button_press!(_button) => {
                self.bus
                    .lock()
                    .unwrap()
                    .send(SceneEvent::GoToTitle)
                    .unwrap();
            }
            _ => (),
        }
    }
}
