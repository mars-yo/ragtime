use std::slice::Iter;
use components::*;

pub struct Player {
    input: Input,
    position: Position,
}

impl Player {
    pub fn new() -> Player {
        Player {
            input: Input::new(),
            position: Default::default(),
        }
    }
    fn update_input(&mut self) {
        self.input.update();
    }
    fn update_position(&mut self) {
        self.position.update((0f32,0f32));
    }
    pub fn updaters() -> [fn(&mut Player);2] {
        [Player::update_input, Player::update_position]
    }

}
