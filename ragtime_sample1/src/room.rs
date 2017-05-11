extern crate ragtime;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use ragtime::room_manager::*;
use ragtime::entity_component::system::*;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct InitRoomInfo {
    recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>,
}

pub struct JoinRoomInfo {
    recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>,
}

impl InitRoomInfo {
    pub fn new(recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>) -> InitRoomInfo {
        InitRoomInfo {
            recv_msg_chan_rx:recv_msg_chan_rx,
        }
    }
}

impl JoinRoomInfo {
    pub fn new(recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>) -> JoinRoomInfo {
        JoinRoomInfo {
            recv_msg_chan_rx:recv_msg_chan_rx,
        }
    }
}

pub struct Sample1Room {
    id: RoomID,
    recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>,
    system: System,
}

impl Room for Sample1Room {
    type InitInfoType = InitRoomInfo;
    type JoinInfoType = JoinRoomInfo;

    fn new(id:RoomID, info:InitRoomInfo) -> Sample1Room {
        println!("new room");
        //systemにいろいろ登録
        //recv_msg_chan_rx:Receiver<(ConnectionID,StringMessage)>,
        Sample1Room {
            id: id,
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
    fn join(&mut self, info:JoinRoomInfo) {
        println!("join room");
    }
    fn id(&self) -> RoomID {
        self.id
    }
}
