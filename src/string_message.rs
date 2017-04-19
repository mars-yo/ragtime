use std::io::{Write, Read, BufReader, BufRead};
use connection_manager::Message;
use std::str::FromStr;

pub struct StringMessage {
    body: String,
}

impl Message for StringMessage {
    fn new() -> StringMessage {
        StringMessage { body: String::new() }
    }
    fn read_from<T: BufRead>(&mut self, reader: &mut T) -> bool {
        reader.read_line(&mut self.body);
        self.body = String::from_str(self.body.trim()).unwrap();
        if self.body.is_empty() { return false; }
        true
    }
    fn clear(&mut self) {
        self.body.clear();
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
