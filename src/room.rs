use macroquad::{
        texture::*,
        text::draw_text,
        shapes::*,
        window::*,
        color::*,
        input::*,
        math::* };
use crate::world::World;
use crate::game::*;

pub type Tile = (u8, u8, u8);

pub struct Room {
    id: u8,
    pub tiles: Vec<Tile>,
    pub pos: Vec2,
    pub col: Color,
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
            col: RED,
            // offset: offset,
        }
    }

    pub fn update (&mut self, world: &World ) {
        // self.pos = -world.to_screen(self.pos);
    }

    pub fn draw ( &self, world: &World, game: &Game ) {
        let pos = -world.to_screen(self.pos);
        macroquad::prelude::draw_rectangle_lines(
            pos.x, pos.y,
            (8*64) as f32, (8*64) as f32, 5.,
            self.col);
        // for tile in &self.tiles {
        //     // let p = -world.to_screen(vec2(tile.1 as f32, tile.2 as f32));
        //
        //     let til_po = vec2((tile.1 as u16 * game.tile_size) as f32
        //                      ,(tile.2 as u16 * game.tile_size) as f32);
        //     // let p   = 
        //     println!("{}", t_p);
        //
        //     // game.draw_tile(tile.0, p.x, p.y
        //     // );
        // }

        draw_text(format!("Room_{}", self.id).as_str(),
            pos.x + 40., pos.y + (8.*64.) + 20., 20., self.col
        );
    }

    pub fn is_hover ( &self ) -> bool {
        let mpos = mouse_position();
        if mpos.0 >= self.pos.x && mpos.0 <= self.pos.x + (8*64) as f32 
        && mpos.1 >= self.pos.y && mpos.1 <= self.pos.y + (8*64) as f32 
        {
            true
        } else 
        {
            false
        }
    }
}

        // for x in (0..128).step_by(8) {
        //     for y in (0..128).step_by(8) {
        //         draw_line(x as f32, y as f32, x as f32+screen_width(), y as f32 +screen_height() , 2., RED);
        //     }
        // }
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
