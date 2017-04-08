use std::io::{Write,Read,BufReader,BufRead};
use sub_component::game_logic::connection_manager::Message;

pub struct StringMessage {
    body:String,
}

impl Message for StringMessage {
    fn new<StringMessage>() -> StringMessage {
        StringMessage {
            body:"".to_string(),
        }
    }
    fn read_from<T:BufRead>(&mut self, reader:&mut T) -> bool {
        true
    }
}
