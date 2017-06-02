extern crate ragtime;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use ragtime::room_manager::*;
use ragtime::entity_component::system::*;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub type PlayerID = u64;

pub struct InitRoomInfo {
    name:String,
}

pub struct JoinRoomInfo {
    recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>,
    player_id:PlayerID,
}

impl InitRoomInfo {
    pub fn new(name:String) -> InitRoomInfo {
        InitRoomInfo {
            name:name,
        }
    }
}

impl JoinRoomInfo {
    pub fn new(player_id:PlayerID, recv_msg_chan_rx:Receiver<MessageOnChannel<StringMessage>>) -> JoinRoomInfo {
        JoinRoomInfo {
            player_id:player_id,
            recv_msg_chan_rx:recv_msg_chan_rx,
        }
    }
}

pub struct Sample1Room {
    id: RoomID,
    name: String,
    players:Vec<(Receiver<MessageOnChannel<StringMessage>>, PlayerID)>,
    system: System,
}

impl Room for Sample1Room {
    type InitInfoType = InitRoomInfo;
    type JoinInfoType = JoinRoomInfo;

    fn new(id:RoomID, info:InitRoomInfo) -> Sample1Room {
        println!("new room");
        Sample1Room {
            id: id,
            name: info.name,
            players: Vec::new(),
            system: System::new(),
        }
    }
    fn update(&mut self) -> Continuance {
        println!("room update");
        for ref elm in self.players.iter_mut() {
            if let Ok(msg) = elm.0.try_recv() {
                println!("room msg {}", msg.1.params()[0]);
            }
        }
        self.system.update();
        //check status
        Continuance::Continue
    }
    fn join(&mut self, info:JoinRoomInfo) {
        println!("join room");
        self.players.push((info.recv_msg_chan_rx, info.player_id));//TODO例外処理
    }
    fn id(&self) -> RoomID {
        self.id
    }
}
