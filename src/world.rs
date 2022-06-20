use crate::room::Room;
use macroquad::math::*;

pub struct World {
    pub origin: Vec2,
    pub rooms: [Option<Room>; 6],
    pub scale: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            origin: Vec2::ZERO,
            rooms: Default::default(),
            scale: 1.,
        }
    }

    pub fn to_screen(&self, pos: Vec2) -> Vec2 {
        pos - self.origin * self.scale
    }

    pub fn to_world(&self, pos: Vec2) -> Vec2 {
        pos / self.scale + self.origin
    }
}
