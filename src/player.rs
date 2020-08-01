use sdl2::event::Event;

use crate::input::Controllers;
use crate::input::Joystick;
use crate::input::WhichJoystick;
use crate::entity::Entity;
use crate::position::Position;
use crate::position::Velocity;
use crate::position::HitMask;
use crate::render::Sprite;

static MAX_SPEED: f32 = 5.0;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            entity: Entity {
                position: Position { x, y },
                velocity: Velocity { dx: 0.0, dy: 0.0 },
                orientation: 0.0,
                sprite: Sprite::Player,
                hitmask: HitMask::Point,
                timeouts: Vec::new(),
            }
        }
    }

    /// Mutate player state based on input events.
    pub fn consume_input(
        &mut self,
        controllers: &impl Controllers,
        event: &Event,
    )  {
        match event {
            Event::JoyAxisMotion { which, axis_idx, .. } => {
                self.consume_joystick_input(
                    &controllers.read_joystick(*which, *axis_idx).unwrap());
            },
            _ => ()
        }
    }
    
    fn consume_joystick_input(
        &mut self,
        joystick: &Joystick
    ) {
        match joystick.which_joystick {
            WhichJoystick::Left => {
                self.entity.velocity.dx = joystick.normal_x() * MAX_SPEED;
                self.entity.velocity.dy = joystick.normal_y() * MAX_SPEED;
            },
            WhichJoystick::Right => {
                if let Some(angle) = joystick.angle() {
                    self.entity.orientation = angle;
                }
            }
        };
    }
}
