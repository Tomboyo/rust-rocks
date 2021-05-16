use std::{
    rc::Rc,
    sync::{mpsc::Sender, Arc, Mutex},
};

use graphics::color;
use opengl_graphics::GlGraphics;
use piston::{
    Button, ButtonArgs, ButtonEvent, ButtonState, ControllerButton, ControllerHat, Event, HatState,
    RenderArgs, UpdateArgs,
};

use crate::{
    component::SpriteKind,
    font::{Font, Layout},
    resource::textures::Textures,
};

use super::{Scene, SceneEvent};

pub struct TitleScene {
    textures: Rc<Textures>,
    font: Rc<Font>,
    bus: Arc<Mutex<Sender<SceneEvent>>>,
    menu: Menu<2>,
}

impl TitleScene {
    pub fn new(
        bus: Arc<Mutex<Sender<SceneEvent>>>,
        textures: Rc<Textures>,
        font: Rc<Font>,
    ) -> Self {
        Self {
            textures,
            font,
            bus,
            menu: Menu {
                choices: [SceneEvent::GoToGame, SceneEvent::Quit],
                cursor_index: 0,
            },
        }
    }
}

impl Scene for TitleScene {
    fn update(&mut self, _args: UpdateArgs) {
        // No-op
    }

    fn render(&mut self, args: RenderArgs, gl: &mut GlGraphics) {
        use graphics::{clear, image, Transformed};

        let texture = self.textures.get(&SpriteKind::Title).unwrap();

        let lines = if self.menu.cursor_index == 0 {
            vec![
                self.font.create_text("> Play <").unwrap(),
                self.font.create_text("Quit").unwrap(),
            ]
        } else {
            vec![
                self.font.create_text("Play").unwrap(),
                self.font.create_text("> Quit <").unwrap(),
            ]
        };

        let [_, _, width, height] = args.viewport().rect;
        let center = (width as f64 / 2.0, height as f64 / 2.0);

        gl.draw(args.viewport(), |c, g| {
            clear([0.0; 4], g);

            image(
                texture,
                c.transform.trans(center.0 - 400.0, center.1 - 300.0),
                g,
            );

            self.font
                .render_lines(&lines, Layout::Centered, center, color::WHITE, &c, g)
                .unwrap();
        });
    }

    fn on_event(&mut self, event: Event) {
        event.button(|args| {
            on_hat_press(args, HatState::Down, || {
                self.menu.move_cursor_down();
            });

            on_hat_press(args, HatState::Up, || {
                self.menu.move_cursor_up();
            });

            on_button_press(args, 0, || {
                self.bus.lock().unwrap().send(self.menu.selected()).unwrap();
            });
        });
    }
}

fn on_hat_press<F>(args: ButtonArgs, state: HatState, callback: F)
where
    F: FnOnce() -> (),
{
    match args {
        ButtonArgs {
            // TODO: possible Piston bug: pressing a hat direction generates the
            // Release event; releasing a hat does nothing.
            state: ButtonState::Release,
            button: Button::Hat(ControllerHat { state: s, .. }),
            ..
        } if s == state => callback(),
        _ => (),
    }
}

fn on_button_press<F>(args: ButtonArgs, button: u8, callback: F)
where
    F: FnOnce() -> (),
{
    match args {
        ButtonArgs {
            state: ButtonState::Press,
            button: Button::Controller(ControllerButton { button: b, .. }),
            ..
        } if b == button => callback(),
        _ => (),
    }
}

#[derive(Copy, Clone)]
struct Menu<const L: usize>
where
    [SceneEvent; L]: Copy,
{
    choices: [SceneEvent; L],
    cursor_index: usize,
}

impl<const L: usize> Menu<L> {
    pub fn move_cursor_down(&mut self) {
        self.cursor_index = (self.cursor_index + 1) % self.choices.len();
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_index == 0 {
            self.cursor_index = self.choices.len() - 1;
        } else {
            self.cursor_index -= 1;
        }
    }

    pub fn selected(&self) -> SceneEvent {
        *self.choices.get(self.cursor_index).unwrap()
    }
}
