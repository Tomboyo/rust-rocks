use sdl2::event::Event;

use crate::input::Controllers;
use crate::input::Joystick;
use crate::input::WhichJoystick;
use crate::position::HitMask;
use crate::position::Position;
use crate::position::Velocity;
use crate::position::{Collidable, DynamicPosition};
use crate::render::Renderable;
use crate::render::Sprite;

static MAX_SPEED: f32 = 5.0;

pub struct Player {
    pub position: Position,
    pub velocity: Velocity,
    pub orientation: f32, // in degrees
    pub sprite: Sprite,
    pub hitmask: HitMask,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Position { x, y },
            velocity: Velocity { dx: 0.0, dy: 0.0 },
            orientation: 0.0,
            sprite: Sprite::Player,
            hitmask: HitMask::Point,
        }
    }

    /// Mutate player state based on input events.
    pub fn consume_input(&mut self, controllers: &impl Controllers, event: &Event) {
        match event {
            Event::JoyAxisMotion {
                which, axis_idx, ..
            } => {
                self.consume_joystick_input(&controllers.read_joystick(*which, *axis_idx).unwrap());
            }
            _ => (),
        }
    }

    fn consume_joystick_input(&mut self, joystick: &Joystick) {
        match joystick.which_joystick {
            WhichJoystick::Left => {
                self.velocity.dx = joystick.normal_x() * MAX_SPEED;
                self.velocity.dy = joystick.normal_y() * MAX_SPEED;
            }
            WhichJoystick::Right => {
                if let Some(angle) = joystick.angle() {
                    self.orientation = angle;
                }
            }
        };
    }
}

impl Collidable for Player {
    fn hit_mask(&self) -> &HitMask {
        &self.hitmask
    }

    fn position(&self) -> &Position {
        &self.position
    }
}

impl<'a> Renderable<'a> for Player {
    fn position(&'a self) -> &'a Position {
        &self.position
    }

    fn orientation(&self) -> f64 {
        self.orientation as f64
    }

    fn sprite(&'a self) -> &'a Sprite {
        &self.sprite
    }
}

impl DynamicPosition for Player {
    fn translate(&mut self) {
        self.position.translate(&self.velocity);
    }

    fn clamp(&mut self, modx: f32, mody: f32) {
        self.position.modulate(modx, mody);
    }
}
