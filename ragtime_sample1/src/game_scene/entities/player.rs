use std::slice::Iter;
use components::*;
use protocol::Protocol;

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
    pub fn update_input(&mut self, protocols: &mut Vec<Protocol>) {
        self.input.update(protocols);
    }
    pub fn update_position(&mut self, protocols: &mut Vec<Protocol>) {
        self.position.update(protocols, (0f32,0f32));
    }
//     pub fn updaters() -> [fn(&mut Player);2] {
//         [Player::update_input, Player::update_position]
//     }

}
