extern crate ragtime;

use ragtime::entity_component::component::*;

pub struct Position {
    pos: (f32, f32),
    move_per_frame: (f32, f32),
}

impl Component for Position {
    fn start(&mut self) {}
    fn update(&mut self) {
        self.pos.0 += self.move_per_frame.0;
        self.pos.1 += self.move_per_frame.1;
    }
}

impl Position {
    pub fn move_to(&mut self, tgt_pos: (f32, f32), after_msec: i32) {}
}
