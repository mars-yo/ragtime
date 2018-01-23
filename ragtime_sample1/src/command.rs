use std::io::BufRead;
use std::str::FromStr;
use ragtime::network::Serialize;


#[derive(Debug)]
pub enum Command {
    None,
    C2S_RequestUIInfo{ ui_id:u32 },
    S2C_ResponseUIInfo{ info: String },
    C2S_Submit{ ui_id:u32, info: String },
    
    Login{ user_id: UserID },
    Join{ user_id: UserID },
    Chat{ message: String },
    Speed((f32,f32)),
    Position((f32,f32)),
}

impl Serialize for Command {
    fn new() -> Command {
        Command::None
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
        if *command == "chat" {
            let msg = v.get(1).unwrap();
            *self = Protocol::Chat{ message:msg };
        }
        true
    }
}
