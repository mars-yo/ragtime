use std::sync::mpsc::Receiver;

use ragtime::game_object_manager::*;
use ragtime::connection_manager::*;
use ragtime::string_message::*;
use components::input::*;
use components::position::*;
use components::typedef;

pub struct Player {
    id: ObjectID,
    input: Input,
    pos: Position,
}

impl Player {
    pub fn new(rx:Receiver<MessageOnChannel<StringMessage>>) -> Player {
        Player {
            id: 0,
            input: Input::new(rx),
            pos: Position::new(),
        }
    }
}

impl GameObject for Player {
    fn id(&self) -> ObjectID {
        self.id
    }
    fn set_id(&mut self, id:ObjectID) {
        self.id = id;
    }
    fn start(&mut self) {
        self.pos.set_input(&self.input);
    }
    fn update_component(&mut self, comp_id:ComponentType) {
        println!("player update {}", comp_id);
        match comp_id {
            typedef::Input => self.input.update(),
            typedef::Position => self.pos.update(),
            _ => panic!(""),
        }
    }
    fn is_zombie(&self) -> bool {
        false
    }

}
