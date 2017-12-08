extern crate ragtime;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use std::rc::Rc;
use std::cell::RefCell;
use game_scene::entities::player::*;
use ragtime::room_manager::*;
use ragtime::connection_manager::*;
use ragtime::string_message::*;
use super::object_manager::*;

pub type PlayerID = u64;

pub struct JoinCommand {
    msg_chan_tx: MsgChanTx<StringMessage>,
    msg_chan_rx: MsgChanRx<StringMessage>,
    player_id:PlayerID,
}

impl JoinCommand {
    pub fn new(player_id:PlayerID, tx: MsgChanTx<StringMessage>, rx: MsgChanRx<StringMessage>) -> JoinCommand {
        JoinCommand {
            player_id:player_id,
            msg_chan_tx:tx,
            msg_chan_rx:rx,
        }
    }
}

pub enum RoomCommand {
    Join(JoinCommand),
}

pub struct Sample1Room {
    id: RoomID,
    name: String,
    object_mgr: ObjectManager,
}

impl Room for Sample1Room {
    type CommandType = RoomCommand;

    fn new(id:RoomID) -> Sample1Room {
        println!("new room");
        Sample1Room {
            id: id,
            name: "".to_string(),
            object_mgr: ObjectManager::new(),
        }
    }
    fn update(&mut self) {
        println!("room update");
        // for ref elm in self.players.iter_mut() {
        //     if let Ok(msg) = elm.0.try_recv() {
        //         println!("room msg {}", msg.1.params()[0]);
        //     }
        // }
        //check status
        self.object_mgr.update();
    }
    fn on_command(&mut self, cmd:&RoomCommand) {
//         match cmd {
//           JoinCommand(cmd) => {
//               // new players
            
//           }
    }
    fn deletable(&self) -> bool {
        false
    }
//     fn join(&mut self, info:JoinRoomInfo) {
//         println!("join room");
//         let p = Player::new(info.recv_msg_chan_rx);
//         self.object_mgr.add_player(p);
//     }
}
