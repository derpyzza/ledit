use crate::room::Room;
use macroquad::math::*;

pub struct World {
    pub offset: Vec2,
    pub rooms: [ Option<Room> ; 8 ],
}

impl World {

    pub fn new () -> Self {
        Self {
            offset: Vec2::ZERO,
            rooms: Default::default(),
        }
    }
    
    pub fn init (&mut self) {
        self.rooms[0] = Some(Room::new(0, vec2(30., 30.)));
    }

    pub fn room_init (&mut self, id: &mut u8, pos: Vec2) {
        self.rooms[*id as usize] = Some(Room::new(0, pos));
        *id += 1;
        println!("created new room at {:?}", pos);
    }

    pub fn to_screen(&self, world: Vec2) -> Vec2 {
        world - self.offset * -1.
    }

    pub fn to_world(&self, screen: Vec2 ) -> Vec2 {
        screen + self.offset
    }
}
