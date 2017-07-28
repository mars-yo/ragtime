extern crate ragtime;

use components::input::*;

pub struct Position {
    input: *const Input,
    pos: (f32, f32),
    move_per_frame: (f32, f32),
}

impl Position {
    fn start(&mut self) {}
    fn update(&mut self) {
        self.pos.0 += self.move_per_frame.0;
        self.pos.1 += self.move_per_frame.1;
        let p = self.input.pos();
    }
}

impl Position {
    pub fn move_to(&mut self, tgt_pos: (f32, f32), after_msec: i32) {}
}
