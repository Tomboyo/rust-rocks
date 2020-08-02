use std::time::Instant;

use sdl2::event::Event;
use sdl2::controller::Button;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::room::Room;
use crate::room::RoomTransition;
use crate::room::Context;

pub struct TitleRoom<'t> {
    menu: Menu<'t>
}

impl <'t> Room for TitleRoom<'t> {
    fn update(
        &mut self,
        _context: &mut Context,
        events: Vec<Event>,
        _now: Instant
    ) -> Option<RoomTransition> {
        events.iter()
            .map(|x| self.consume_input(x))
            .find(Option::is_some)
            .flatten()
    }

    fn render(
        &self,
        context: &mut Context
    ) {
        context.canvas.clear();

        context.canvas.copy(
            &context.textures.title,
            None,
            None).expect("Failed to render title!");
        
        self.menu.render(context.canvas);
        
        context.canvas.present();
    }
}

impl <'t> TitleRoom<'t> {
    pub fn new(
        font: &Font,
        texture_creator: &'t TextureCreator<WindowContext>,
    ) -> Self {
        TitleRoom {
            menu: Menu::new(
                LogicalMenu::new(
                    vec![
                        RoomTransition::Game,
                        RoomTransition::Quit,
                    ]),
                font,
                texture_creator),
        }
    }

    fn consume_input(
        &mut self,
        event: &Event
    ) -> Option<RoomTransition> {
        match event {
            Event::ControllerButtonDown { button, .. }
            if *button == Button::DPadDown => {
                self.menu.logical.move_cursor_down();
                None
            },
            Event::ControllerButtonDown { button, .. }
            if *button == Button::DPadUp => {
                self.menu.logical.move_cursor_up();
                None
            },
            Event::ControllerButtonDown { button, .. }
            if *button == Button::A => {
                Some(self.menu.logical.selected())
            }
            _ => None
        }
    }
}

struct LogicalMenu {
    choices: Vec<RoomTransition>,
    cursor_index: usize,
}

struct Menu<'t> {
    logical: LogicalMenu,
    selected_textures: Vec<Texture<'t>>,
    unselected_textures: Vec<Texture<'t>>
}

impl LogicalMenu {
    pub fn new(
        choices: Vec<RoomTransition>
    ) -> Self {
        if choices.len() < 1 {
            panic!("At least one choice is required");
        }

        Self {
            choices,
            cursor_index: 0,
        }
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_index =
            (self.cursor_index + 1)
            % self.choices.len();
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_index == 0 {
            self.cursor_index = self.choices.len() - 1;
        } else {
            self.cursor_index -= 1;
        }
    }

    pub fn selected(&self) -> RoomTransition {
        *self.choices.get(self.cursor_index).unwrap()
    }
}

impl <'t> Menu<'t> {
    pub fn new(
        logical: LogicalMenu,
        font: &Font,
        texture_creator: &'t TextureCreator<WindowContext>,
    ) -> Self {
        let selected_textures =
            logical.choices.iter().map(|x| match x {
                RoomTransition::Game => "> Start",
                RoomTransition::Quit => "> Exit",
            })
            .map(|x| Self::create_texture(x, font, texture_creator))
            .collect();
            
        let unselected_textures =
            logical.choices.iter().map(|x| match x {
                RoomTransition::Game => "  Start",
                RoomTransition::Quit => "  Exit",
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
        texture_creator: &'t TextureCreator<WindowContext>
    ) -> Texture<'t> {
        font.render(text)
            .solid(Color::WHITE)
            .expect("Failed to render menu")
            .as_texture(&texture_creator)
            .expect("Failed to texturize menu surface")
    }

    fn render(
        &self,
        canvas: &mut Canvas<Window>
    ) {
        let max_texture_width =
            self.selected_textures.iter().chain(
                self.unselected_textures.iter())
            .map(|x| x.query().width)
            .max()
            .unwrap();
        
        self.logical.choices.iter().enumerate().for_each(|(index, _)| {
            let t =
                if index == self.logical.cursor_index {
                    self.selected_textures.get(index).unwrap()
                } else {
                    self.unselected_textures.get(index).unwrap()
                };
            
            let query = t.query();
            let (window_width, window_height) = canvas.window().size();
            canvas.copy(
                &t,
                None,
                sdl2::rect::Rect::new(
                    // text is centered on the x-axis
                    ((window_width / 2) - (max_texture_width / 2)) as i32,
                    // menu is rendered 2/3 down the window
                    (window_height as f32 * (2.0 / 3.0)) as i32
                        + (query.height * index as u32) as i32,
                    query.width,
                    query.height))
                .expect("Failed to copy font texture to canvas");
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_menu_navigation() {
        let mut menu = LogicalMenu::new(vec![
            RoomTransition::Game,
            RoomTransition::Quit
        ]);

        assert_eq!(
            RoomTransition::Game,
            menu.selected(),
            "The cursor starts on the first option");
        
        // Scroll up through all choices
        (0..menu.choices.len()).for_each(|_| {
            menu.move_cursor_up();
        });
        assert_eq!(
            RoomTransition::Game,
            menu.selected(),
            "The cursor wraps around the top");
        
        // Scroll down through all events
        (0..menu.choices.len()).for_each(|_| {
            menu.move_cursor_down();
        });
        assert_eq!(
            RoomTransition::Game,
            menu.selected(),
            "The cursor wraps around the bottom");
    }
}
