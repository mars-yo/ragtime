use std::slice::Iter;
use components::*;
use protocol::Protocol;

#[derive(Default)]
pub struct Obstacle {
    position: Position,
}

impl Obstacle {
    fn new() -> Obstacle {
        Obstacle{ ..Default::default() }
    }
    
    pub fn update_input(&mut self, protocols: &mut Vec<Protocol>) {
    }
    pub fn update_position(&mut self, protocols: &mut Vec<Protocol>) {
        self.position.update(protocols, (0f32,0f32));
    }
//     pub fn updaters() -> [fn(&mut Obstacle);2] {
//         [Obstacle::update_input, Obstacle::update_position]
//     }

}
