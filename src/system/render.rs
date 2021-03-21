use std::{rc::Rc, sync::Mutex};

use legion::{system, world::SubWorld, IntoQuery};
use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::{
    component::{Orientation, Position, Sprite},
    resource::textures::Textures,
};

#[system]
#[read_component(Position)]
#[read_component(Orientation)]
#[read_component(Sprite)]
pub fn render(
    world: &mut SubWorld,
    #[resource] canvas: &mut Rc<Mutex<Canvas<Window>>>,
    #[resource] tm: &Rc<Textures>,
) {
    let mut canvas = canvas.lock().unwrap();

    canvas.clear();

    let mut query = <(&Position, &Orientation, &Sprite)>::query();
    for (position, orientation, sprite) in query.iter_mut(world) {
        let texture = tm.get_texture(&sprite.kind);

        let query = texture.query();
        let destination = Rect::new(
            (position.x - (query.width as f32 / 2.0)) as i32,
            (position.y - (query.height as f32 / 2.0)) as i32,
            query.width,
            query.height,
        );
        canvas
            .copy_ex(
                &texture,
                None,
                destination,
                orientation.0 as f64,
                None, // rotate around center of `destination`
                false,
                false,
            )
            .unwrap();
    }

    canvas.present();
}
