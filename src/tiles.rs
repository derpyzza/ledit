// tile implementation. could move to room.rs too actually.
use crate::room::Room;

pub struct Tile(pub u8, pub u8, pub u8); // tile structure.
                                         // holds the following: texture id, tile column, tile row.

impl Tile {
    /// Create and return a new Tile struct
    pub fn new(_index: u8, _row: u8, _column: u8) -> Self {
        Self(_index, _row, _column)
    }
    /// Translate cell coordinates to screen coordinates
    pub fn to_world(&self) -> (f32, f32) {
        let pos = (
            self.1 as f32 * 32. + 32.,
            self.2 as f32 * 32. + 32.,
        );
        pos
    }
    pub fn to_cell(coords: (f32, f32), room: &Room) -> (u8, u8) {
        let pos = (
            (coords.0 / 32.) as u8,
            (coords.1 / 32.) as u8,
        );
        pos
    }
}
