use macroquad::{
    math::*,
    shapes::draw_rectangle_lines,
    color::*,
    texture::*,
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

    pub fn draw(&self, tilemap: Texture2D) {
        // draw_rectangle_lines(pos.x, pos.y, 16. * 32., 16. * 32., 3., RED);
        for tile in &self.tiles {
            draw_texture_ex(
                tilemap,
                tile.1 as f32 * 48.,
                tile.2 as f32 * 48.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(48., 48.)),

                    source: Some(Rect::new(
                        tile.0 as f32 * 16.,
                        0.,
                        16.,
                        16.,
                    )),

                    ..Default::default()
                },
            );
        }
    }

    pub fn push_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }
}
