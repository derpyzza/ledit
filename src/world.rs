use crate::room::Room;
use macroquad::math::*;

pub struct World {
    pub origin: Vec2,
    pub rooms: [Option<Room>; 6],
}

impl World {
    pub fn new() -> Self {
        Self {
            origin: Vec2::ZERO,
            rooms: Default::default(),
        }
    }

    pub fn to_screen(&self, pos: Vec2) -> Vec2 {
        pos - self.origin
    }

    pub fn to_world(&self, pos: Vec2) -> Vec2 {
        pos + self.origin
    }

    pub fn is_hover(&self, room: &Room) {}
}

// pub fn init (&mut self) {
//     self.rooms[0] = Some(Room::new(0, vec2(30., 30.)));
// }

// pub fn room_init (&mut self, id: &mut u8, pos: Vec2) {
//     self.rooms[*id as usize] = Some(Room::new(0, pos));
//     *id += 1;
//     println!("created new room at {:?}", pos);
// }
