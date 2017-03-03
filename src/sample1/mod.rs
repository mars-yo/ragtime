use entity_component::component::*;
use entity_component::entity::*;
use entity_component::system::*;

enum Sample1Components {

}

impl SubComponent for Sample1Components {
    fn start(&mut self) {

    }
    fn update(&mut self) {

    }
}

struct Sample1Game {
    system:System<Sample1Components>,
}

pub fn sample1_start() {
    //make game AsMut
    //add connection manager
    //add game object manager

}
