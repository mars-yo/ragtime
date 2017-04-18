use sub_component::game_logic::connection_manager::*;
use string_message::*;

pub struct Input {}

impl Input {
    pub fn start(&mut self) {}
    pub fn update(&mut self) {}
    pub fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {}
}
