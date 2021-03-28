use std::{rc::Rc, sync::Mutex};

use legion::{system, world::SubWorld, IntoQuery};
use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::{
    component::{Spatial, Sprite},
    resource::textures::Textures,
};

#[system]
#[read_component(Spatial)]
#[read_component(Sprite)]
pub fn render(
    world: &mut SubWorld,
    #[resource] canvas: &mut Rc<Mutex<Canvas<Window>>>,
    #[resource] tm: &Rc<Textures>,
) {
    let mut canvas = canvas.lock().unwrap();

    canvas.clear();

    for (spatial, sprite) in <(&Spatial, &Sprite)>::query().iter(world) {
        let texture = tm.get_texture(&sprite.kind);

        let query = texture.query();
        let destination = Rect::new(
            (spatial.x - (query.width as f32 / 2.0)) as i32,
            (spatial.y - (query.height as f32 / 2.0)) as i32,
            query.width,
            query.height,
        );
        canvas
            .copy_ex(
                &texture,
                None,
                destination,
                spatial.angle_o.to_degrees() as f64,
                None, // rotate around center of `destination`
                false,
                false,
            )
            .unwrap();
    }

    canvas.present();
}
