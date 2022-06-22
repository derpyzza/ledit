use macroquad::{
    math::*,
    shapes::draw_rectangle_lines,
    color::RED
};
use crate::tile::Tile;

pub struct Room {
    id: u8,
    tiles: Vec<Tile>,
    size: f32,
}

impl Room {
    pub fn new( _id: u8 ) -> Self {
        Self {
            id: _id,
            tiles: Vec::new(),
            size: 48. * 32.
        }
    }

    pub fn draw(&self, pos: Vec2) {
        draw_rectangle_lines(pos.x, pos.y, 16. * 32., 16. * 32., 3., RED);
    }
}
