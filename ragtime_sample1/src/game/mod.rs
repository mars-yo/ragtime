extern crate ragtime;

use ragtime::room;

pub type PlayerID = u64;

pub struct Sample1Room {
}

impl room::Room for Sample1Room {
    type CommandType = String;

    fn new(id:room::RoomID) -> Sample1Room {
        println!("new room");
        Sample1Room {
        }
    }
    fn update(&mut self, commands:Vec<String>) {
        println!("room update");
    }
    fn deletable(&self) -> bool {
        false
    }
}
