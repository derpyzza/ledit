use::macroquad::
{
    texture::*,
    color::*,
    math::*,
};

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

    pub fn draw_tile ( &self, tile: u8, x: f32, y: f32 ) {
        draw_texture_ex(self.tilemap,
            x,
            y,
            WHITE,
            DrawTextureParams
            {
                dest_size: Some(vec2(32.0, 32.0)),
                source: Some(Rect::new((tile*self.tile_size) as f32, 0., 8., 8.)),
                ..Default::default()
        });
    }
}
