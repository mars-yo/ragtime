extern crate ragtime;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use std::rc::Rc;
use std::cell::RefCell;
use game_scene::entities::player::*;
use game_scene::common_entities::*;
use game_scene::personal_entities::*;
use ragtime::network::*;
use super::*;
use super::super::*;

pub type PlayerID = u64;

pub struct JoinCommand {
    player_id:PlayerID,
    msg_tx: MsgTx,
    msg_rx: MsgRx,
}

impl JoinCommand {
    pub fn new(player_id:PlayerID, tx: MsgTx, rx: MsgRx) -> JoinCommand {
        JoinCommand {
            player_id:player_id,
            msg_tx:tx,
            msg_rx:rx,
        }
    }
}

pub enum RoomCommand {
    Join(JoinCommand),
}

pub struct Sample1Room {
    id: RoomID,
    name: String,
    common_entities: CommonEntities,
    personal_entities: HashMap<PlayerID,PersonalEntities>,
    players: HashMap<PlayerID, (MsgTx,MsgRx)>,
}

impl Room for Sample1Room {
    type CommandType = RoomCommand;

    fn new(id:RoomID) -> Sample1Room {
        println!("new room");
        Sample1Room {
            id: id,
            name: "".to_string(),
            common_entities: CommonEntities::new(),
            personal_entities: HashMap::new(),
            players: HashMap::new(),
        }
    }
    fn update(&mut self) {
        println!("room update");

        let mut msgs:HashMap<PlayerID,Vec<Protocol>> = HashMap::new();
        for (player_id, ref mut chan) in self.players.iter_mut() {
            let rx = &chan.1;
            if let Ok(msg) = rx.try_recv() {
                println!("room msg {:?}", msg);
                
                let (conn_id, msg) = *msg;
                
                if msgs.contains_key(&player_id) {
                    let v = msgs.get_mut(&player_id).unwrap();
                    v.push(msg);
                } else {
                    let mut v = Vec::new();
                    v.push(msg);
                    msgs.insert( *player_id, v );
                }
            }
        }

        self.common_entities.update(&msgs);
        for (player_id, msg) in msgs.iter_mut() {
            if let Some(ents) = self.personal_entities.get_mut(player_id) {
                ents.update(&msg);  
            }  
        }
    }
    fn on_command(&mut self, cmd:&RoomCommand) {
//         match cmd {
//           RoomCommand::Join(cmd) => {
              // new players
              println!("join");
            
//           }
//         }
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
