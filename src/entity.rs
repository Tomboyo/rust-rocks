use sdl2::render::Texture;

pub struct Entity<'a> {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub orientation: f32,
    texture: Texture<'a>,
}

// TODO: remove pub!
pub struct System<'a> {
    pub player: Entity<'a>,
    pub asteroids: Vec<Entity<'a>>,
    pub bullets: Vec<Entity<'a>>,
}

// orientatin is in degrees
impl <'a> Entity<'a> {
    pub fn new(
        x: i32, y: i32,
        dx: f32, dy: f32,
        orientation: f32,
        texture: Texture
    ) -> Entity {
        Entity {
            x: x as f32,
            y: y as f32,
            dx,
            dy,
            orientation,
            texture: texture,
        }
    }

    pub fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.texture.query().width
    }

    pub fn height(&self) -> u32 {
        self.texture.query().height
    }

    pub fn orientation_rad(&self) -> f32 {
        self.orientation * std::f32::consts::PI / 180.0
    }

    pub fn orientation_deg(&self) -> f32 {
        self.orientation
    }
}

impl <'a> System<'a> {
    pub fn new(
        player: Entity<'a>,
        asteroids: Vec<Entity<'a>>,
        bullets: Vec<Entity<'a>>,
    ) -> System<'a> {
        System {
            player,
            asteroids,
            bullets
        }
    }
}
