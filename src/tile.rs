
pub struct Tile(pub u8, pub u8, pub u8);

impl Tile {
    pub fn new ( _index: u8, _row: u8, _column: u8 ) -> Self {
        Self(_index, _row, _column)
    }
}
