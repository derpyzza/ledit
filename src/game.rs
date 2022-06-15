use::macroquad::texture::*;

pub struct Game {
    pub tilemap:    Texture2D,
    pub tile_size:  u8,
}

impl Game {
    pub fn new (tilemap: &Texture2D , tile_size: u8) -> Self {
        Self {
            tilemap: *tilemap,
            tile_size: tile_size
        }
    }
}
