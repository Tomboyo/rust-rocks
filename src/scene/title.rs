use std::{
    rc::Rc,
    sync::{mpsc::Sender, Mutex},
};

use sdl2::controller::Button;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::{
    component::SpriteKind,
    resource::{input_events::InputEvents, textures::Textures},
};

use super::{scene_event::SceneEvent, Scene};

pub struct TitleScene {
    canvas: Rc<Mutex<Canvas<Window>>>,
    textures: Rc<Textures>,
    bus: Rc<Sender<SceneEvent>>,
    menu: Menu,
}

impl Scene for TitleScene {
    fn run(&mut self, events: InputEvents) {
        self.update_menu(&events);
        self.get_selection(&events)
            .map(|event| self.bus.send(event));
        self.render()
    }
}

impl TitleScene {
    pub fn new(
        canvas: Rc<Mutex<Canvas<Window>>>,
        textures: Rc<Textures>,
        bus: Rc<Sender<SceneEvent>>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Self {
        let menu = Menu::new(LogicalMenu::new(), font, texture_creator);
        Self {
            canvas,
            textures,
            bus,
            menu,
        }
    }

    fn update_menu(&mut self, events: &InputEvents) {
        events.iter().for_each(|event| match event {
            Event::ControllerButtonDown { button, .. } if *button == Button::DPadDown => {
                self.menu.logical.move_cursor_down();
            }
            Event::ControllerButtonDown { button, .. } if *button == Button::DPadUp => {
                self.menu.logical.move_cursor_up();
            }
            _ => {}
        });
    }

    fn get_selection(&self, events: &InputEvents) -> Option<SceneEvent> {
        events
            .iter()
            .find(|event| {
                matches!(
                    event,
                    Event::ControllerButtonDown {
                        button: Button::A,
                        ..
                    }
                )
            })
            .map(|_| self.menu.logical.selected())
    }

    fn render(&mut self) {
        let mut canvas = self.canvas.lock().unwrap();
        canvas.clear();

        canvas
            .copy(self.textures.get_texture(&SpriteKind::Title), None, None)
            .expect("Failed to render title!");

        self.menu.render(&mut canvas);

        canvas.present();
    }
}

struct LogicalMenu {
    choices: Vec<SceneEvent>,
    cursor_index: usize,
}

struct Menu {
    logical: LogicalMenu,
    selected_textures: Vec<Texture>,
    unselected_textures: Vec<Texture>,
}

impl LogicalMenu {
    pub fn new() -> Self {
        Self {
            choices: vec![SceneEvent::GoToGame, SceneEvent::Quit],
            cursor_index: 0,
        }
    }

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

impl Menu {
    pub fn new(
        logical: LogicalMenu,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Self {
        let selected_textures = logical
            .choices
            .iter()
            .map(|x| match x {
                SceneEvent::GoToGame => "> Start",
                SceneEvent::Quit => "> Exit",
            })
            .map(|x| Self::create_texture(x, font, texture_creator))
            .collect();

        let unselected_textures = logical
            .choices
            .iter()
            .map(|x| match x {
                SceneEvent::GoToGame => "  Start",
                SceneEvent::Quit => "  Exit",
            })
            .map(|x| Self::create_texture(x, font, texture_creator))
            .collect();

        Menu {
            logical,
            selected_textures,
            unselected_textures,
        }
    }

    fn create_texture(
        text: &str,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Texture {
        font.render(text)
            .solid(Color::WHITE)
            .expect("Failed to render menu")
            .as_texture(&texture_creator)
            .expect("Failed to texturize menu surface")
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        let max_texture_width = self
            .selected_textures
            .iter()
            .chain(self.unselected_textures.iter())
            .map(|x| x.query().width)
            .max()
            .unwrap();

        self.logical
            .choices
            .iter()
            .enumerate()
            .for_each(|(index, _)| {
                let t = if index == self.logical.cursor_index {
                    self.selected_textures.get(index).unwrap()
                } else {
                    self.unselected_textures.get(index).unwrap()
                };

                let query = t.query();
                let (window_width, window_height) = canvas.window().size();
                canvas
                    .copy(
                        &t,
                        None,
                        sdl2::rect::Rect::new(
                            // text is centered on the x-axis
                            ((window_width / 2) - (max_texture_width / 2)) as i32,
                            // menu is rendered 2/3 down the window
                            (window_height as f32 * (2.0 / 3.0)) as i32
                                + (query.height * index as u32) as i32,
                            query.width,
                            query.height,
                        ),
                    )
                    .expect("Failed to copy font texture to canvas");
            });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_menu_navigation() {
        let mut menu = LogicalMenu::new();

        assert_eq!(
            SceneEvent::GoToGame,
            menu.selected(),
            "The cursor starts on the first option"
        );

        // Scroll up through all choices
        (0..menu.choices.len()).for_each(|_| {
            menu.move_cursor_up();
        });
        assert_eq!(
            SceneEvent::GoToGame,
            menu.selected(),
            "The cursor wraps around the top"
        );

        // Scroll down through all events
        (0..menu.choices.len()).for_each(|_| {
            menu.move_cursor_down();
        });
        assert_eq!(
            SceneEvent::GoToGame,
            menu.selected(),
            "The cursor wraps around the bottom"
        );
    }
}
