use macroquad::{
      texture::*,
         text::draw_text,
       shapes::*,
       window::*,
        color::*,
        input::*,
         math::*,
};
use crate::world::World;
use crate::game::*;

pub type Tile = (u8, u8, u8);

pub struct Room {

    id: u8,
    pub tiles: Vec<Tile>, // make private. only rooms should be allowed to manage their rooms.
    pub pos: Vec2,
    pub col: Color,
    size: f32,

}

impl Room {

    pub fn new ( id: u8, position: Vec2 ) -> Self {

        Self {

            id: id,
            tiles: Vec::new(),
            pos: position,
            col: RED,
            size: 32.,

        }

    }

    pub fn update (&mut self, world: &World ) {

    }

    pub fn draw ( &self, world: &World, game: &Game ) {

        let pos = -world.to_screen(self.pos);
        let tile_size = game._onscreen_size();

        macroquad::prelude::draw_rectangle_lines(
            pos.x, pos.y,
            tile_size*self.size, tile_size*self.size, 5.,
            self.col);

        for tile in &self.tiles {

            // get tile data in cells, transform that to screen coords.
            let tile_pos: Vec2 = {

                let mut level_coords = vec2( // transform cell data to grid coords.
                    tile.1 as f32 * tile_size,
                    tile.2 as f32 * tile_size,
                );

                level_coords -= self.pos; // transform to local level coords
                level_coords = world.to_world(level_coords); // transform to world coords
                level_coords
            };

            game.draw_tile(tile.0, tile_pos.x, tile_pos.y);

        }

        draw_text(format!("Room_{}", self.id).as_str(),
            pos.x + 40., pos.y + (tile_size*self.size) + 20., 20., self.col
        );

    }

    // pub fn is_hover ( &self ) -> bool {
    //
    //     let mpos = mouse_position();
    //     // let tile_size = game._tile_size();
    //
    //     if mpos.0 >= self.pos.x && mpos.0 <= self.pos.x + 
    //     && mpos.1 >= self.pos.y && mpos.1 <= self.pos.y +  
    //     {
    //         true
    //     } else 
    //     {
    //         false
    //     }
    //
    // }


        // if self.is_hover() { 
        //     println!("yay");
        //      // get mouse coords and translate to screen 
        //     let m_pos = {
        //          let mut smp = vec1(mouse_position().0, mouse_position().1);
        //          // smp -= world.origin;
        //          // smp -= self.pos;
        //
        //          smp
        //     };
        //     draw_rectangle_lines(m_pos.x, m_pos.y, tile_size, tile_size, 3., RED);
        // }

}
