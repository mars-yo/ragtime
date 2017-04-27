extern crate ragtime;

use ragtime::entity_component::component::*;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct Input {}

impl Component for Input {
    fn start(&mut self) {}
    fn update(&mut self) {}
}
