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
        println!("new_room");
        Sample1RoomInitializeInfo {
            recv_msg_chan_rx:recv_msg_chan_rx,
        }
    }
}

pub struct Sample1Room {
    recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>,
    system: System,
}

impl Room for Sample1Room {
    type InitializeInfo = Sample1RoomInitializeInfo;

    fn new(info:Sample1RoomInitializeInfo) -> Sample1Room {
        //systemにいろいろ登録
        //recv_msg_chan_rx:Receiver<(ConnectionID,StringMessage)>,
        Sample1Room {
            recv_msg_chan_rx: info.recv_msg_chan_rx,
            system: System::new(),
        }
    }
    fn update(&mut self) -> Continuance {
        println!("room update");
        if let Ok(msg) = self.recv_msg_chan_rx.try_recv() {
            println!("room msg {}", msg.1.params()[0]);
        }
        self.system.update();
        //check status
        Continuance::Continue
    }
}
