extern crate ragtime;
use std::ptr;
use components::input::*;

pub struct Position {
    input: *const Input,
    pos: (f32, f32),
    move_per_frame: (f32, f32),
}

impl Position {
    pub fn new() -> Position {
        Position {
            input: ptr::null(),
            pos: (0f32,0f32),
            move_per_frame: (0f32,0f32),
        }
    }
    pub fn set_input(&mut self, inp: *const Input) {
        self.input = inp;
    }
    fn input<'a>(&self) -> &'a Input {
        unsafe {
            &(*self.input)
        }
    }
    pub fn update(&mut self) {
        self.pos.0 += self.input().moving().0;
        self.pos.1 += self.input().moving().1;
    }
}

impl Position {
    pub fn move_to(&mut self, tgt_pos: (f32, f32), after_msec: i32) {}
}
