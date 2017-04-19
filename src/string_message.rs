use std::io::{Write, Read, BufReader, BufRead};
use sub_component::game_logic::connection_manager::Message;

pub struct StringMessage {
    body: String,
}

impl Message for StringMessage {
    fn new() -> StringMessage {
        StringMessage { body: String::new() }
    }
    fn read_from<T: BufRead>(&mut self, reader: &mut T) -> bool {
        reader.read_to_string(&mut self.body);
        true
    }
}

impl StringMessage {
    pub fn body(&self) -> &String {
        &self.body
    }
    pub fn params(&self) -> Vec<&str> {
        self.body.split(',').collect()
    }
}
