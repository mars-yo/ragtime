use std::io::BufRead;
use std::str::FromStr;
use ragtime::network::Message;

#[derive(Debug)]
pub enum Protocol {
    None,
    Speed((f32,f32)),
    Position((f32,f32)),
}

impl Message for Protocol {
    fn new() -> Protocol {
        Protocol::None
    }
    fn read_from<T: BufRead>(&mut self, reader: &mut T) -> bool {
        let mut body = String::new();
        reader.read_line(&mut body);
        let mut body = String::from_str(body.trim()).unwrap();
        if body.is_empty() { return false; }

        let v:Vec<&str> = body.split(',').collect();
        if v.is_empty() { return false; }
        
        let command = v.get(0).unwrap();
        println!("{:?}", command);
        if *command == "speed" {
            *self = Protocol::Speed((0f32,0f32));
        }
        true
    }
}
