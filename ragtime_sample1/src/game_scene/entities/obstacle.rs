use std::slice::Iter;
use components::*;

#[derive(Default)]
pub struct Obstacle {
    position: Position,
}

impl Obstacle {
    fn new() -> Obstacle {
        Obstacle{ ..Default::default() }
    }
    
    fn update_input(&mut self) {
    }
    fn update_position(&mut self) {
        self.position.update((0f32,0f32));
    }
    pub fn updaters() -> [fn(&mut Obstacle);2] {
        [Obstacle::update_input, Obstacle::update_position]
    }

}
