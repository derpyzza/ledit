use macroquad::{
        texture::*,
        text::draw_text,
        color::*,
        math::Vec2 };
use crate::game::*;

pub type Tile = (u8, f32, f32);

pub struct Room {
    pub id: u8,
    tiles: Vec<Tile>,
    pub pos: Vec2,
    // offset: Vec2,
}

impl Room {

    // pub fn to_world (&self, room: Vec2) -> Vec2 {
    //     room + self.offset
    // }
    //
    // pub fn to_room(&self, world: Vec2) -> Vec2 {
    //     world - self.offset
    // }
    pub fn new ( id: u8, position: Vec2 ) -> Self {
        Self {
            id: id,
            tiles: Vec::new(),
            pos: position,
            // offset: offset,
        }
    }
    pub fn draw ( &self, game: &Game, origin: Vec2 ) {
        macroquad::prelude::draw_rectangle_lines(
            self.pos.x + origin.x, self.pos.y + origin.y, 
            (8*64) as f32, (8*64) as f32, 5.,
            macroquad::prelude::RED);
        // println!("drawing room_{} at ({}, {})", self.id, self.pos.x, self.pos.y);
        // draw_text(  format!("ROOM_{}", self.id).as_str(),
        //             self.pos.x,
        //             self.pos.y,
        //             40.,
        //             RED
        // )
        // draw_texture_ex(game.tilemap,
        // coords.x,
        // coords.y, WHITE, 
        // DrawTextureParams
        // {
        //     dest_size: Some(vec2(32.0, 32.0)),
        //     source: Some(Rect::new((tile*game.tile_size) as f32, 0., 8., 8.)),
        //     ..Default::default()
        // });
    }
}
