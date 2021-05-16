mod game;
mod scene_event;
mod score;
mod title;

use opengl_graphics::GlGraphics;
use piston::{Event, RenderArgs, UpdateArgs};

pub use game::GameScene;
pub use scene_event::SceneEvent;
pub use score::ScoreScene;
pub use title::TitleScene;

pub trait Scene {
    fn update(&mut self, args: UpdateArgs);

    fn render(&mut self, args: RenderArgs, gl: &mut GlGraphics);

    fn on_event(&mut self, event: Event);
}
