// impl<'a> Textures<'a> {
//     pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Textures {
//         Textures {
//             asteroid: Self::load_texture(texture_creator, "asteroid.bmp"),
//             bullet: Self::load_texture(texture_creator, "bullet.bmp"),
//             player: Self::load_texture(texture_creator, "player-ship.bmp"),
//             title: Self::load_texture(texture_creator, "title.bmp"),
//         }
//     }
// }
// pub trait Renderable<'a> {
//     fn position(&'a self) -> &'a Position;
//     fn orientation(&self) -> f64;
//     fn sprite(&'a self) -> &'a Sprite;
// }

// pub fn render<'a>(
//     canvas: &mut Canvas<Window>,
//     textures: &Textures,
//     renderable: &'a dyn Renderable<'a>,
// ) -> Result<(), String> {
// let position = renderable.position();
// let orientation = renderable.orientation();
// let texture = textures.get_texture(renderable.sprite());
// let query = texture.query();
// let destination = Rect::new(
//     (position.x - (query.width as f32 / 2.0)) as i32,
//     (position.y - (query.height as f32 / 2.0)) as i32,
//     query.width,
//     query.height,
// );
// canvas.copy_ex(
//     &texture,
//     None,
//     destination,
//     orientation as f64,
//     None, // rotate around center of `destination`
//     false,
//     false,
// )
// }
