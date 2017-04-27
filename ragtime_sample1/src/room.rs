extern crate ragtime;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use ragtime::room_manager::*;
use ragtime::entity_component::system::*;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct Sample1RoomInitializeInfo {
    recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>,
}

impl Sample1RoomInitializeInfo {
    pub fn new(recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>) -> Sample1RoomInitializeInfo {
        Sample1RoomInitializeInfo {
            recv_msg_chan_rx:recv_msg_chan_rx,
        }
    }
}

pub struct Sample1Room {
    system: System,
}

impl Room for Sample1Room {
    type InitializeInfo = Sample1RoomInitializeInfo;

    fn new(info:&Sample1RoomInitializeInfo) -> Sample1Room {
        //systemにいろいろ登録
        //recv_msg_chan_rx:Receiver<(ConnectionID,StringMessage)>,
        Sample1Room {
            system: System::new(),
        }
    }
    fn update(&mut self) -> Continuance {
        self.system.update();
        //check status
        Continuance::Continue
    }
}
