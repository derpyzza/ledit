use ::macroquad::{color::*, math::*, texture::*};

use crate::{room::Room, world::World};

// NOTE
// native tile size ( game.tile_size ) is for grabbing the tile from the tilemap only.
// onscreen tile size is for everything related to drawing and representing grids

pub struct Game {
    tilemap: Texture2D,
    tile_size: u8,
    onscreen_tile_size: f32,
}

impl Game {
    pub fn new(tilemap: &Texture2D, tile_size: u8, onscreen_tile_size: f32) -> Self {
        Self {
            tilemap: *tilemap,
            tile_size: tile_size,
            onscreen_tile_size: onscreen_tile_size,
        }
    }

    pub fn _onscreen_size(&self) -> f32 {
        self.onscreen_tile_size
    }

    pub fn _tile_size(&self) -> u8 {
        self.tile_size
    }

    // TODO don't draw tiles outside of the room's boundaries
    pub fn draw_tile(&self, tile: u8, x: f32, y: f32, room: &Room, world: &World) {
        // if tile.x > self.tile_size *

        if x < room.pos.x - self.onscreen_tile_size
            || y < room.pos.y - self.onscreen_tile_size
            || x > world.origin.x + room.pos.x + (room.size * self.onscreen_tile_size)
            || y > world.origin.y + room.pos.y + (room.size * self.onscreen_tile_size)
        {
            return;
        }

        draw_texture_ex(
            self.tilemap,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.onscreen_tile_size, self.onscreen_tile_size)),

                source: Some(Rect::new(
                    (tile * self.tile_size) as f32,
                    0.,
                    self.tile_size as f32,
                    self.tile_size as f32,
                )),

                ..Default::default()
            },
        );
    }
}
