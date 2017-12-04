#[macro_use]
extern crate ragtime;

mod components;
mod receptor;
mod game_scene;

use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use std::sync::mpsc::{channel,Receiver,Sender};
use receptor::*;

use game_scene::room::*;

use ragtime::connection_manager::*;
use ragtime::db_manager::DBManager;
use ragtime::game_object_manager::*;
use ragtime::room_manager::*;
use ragtime::string_message::*;

struct Sample1Game {
    room_manager:RoomManager<Sample1Room>,
    conn_mgr: ConnectionManager<StringMessage>,
    db_manager: DBManager,
    new_conn: Vec<(MsgChanTx<StringMessage>,MsgChanRx<StringMessage>)>,
}

impl Sample1Game {
    fn new() -> Sample1Game {

        let mut conn = ConnectionManager::<StringMessage>::new("127.0.0.1:8080".to_string());

        let room_man = RoomManager::<Sample1Room>::new(2);
        let db = DBManager::new();

        Sample1Game {
            room_manager: room_man,
            conn_mgr: conn,
            db_manager: db,
            players: HashMap::new(),
        }
    }
    fn update(&mut self) {
        //recv msg from chann, create room when requested, join room when requested,
        let new_conn = self.conn_mgr.poll();
      
        if let Some(tx,rx) = new_conn {
            self.new_conn.push((tx,rx));
        }

//         if let Ok(msg) = self.receptor_recv_msg_chan_rx.try_recv() {
//             let conn_id = msg.0;
//             let msg = msg.1;
//             if msg.params()[0] == "create_room" {
//                 println!("create_room");
//                 let player_id = msg.params()[1].parse::<PlayerID>().unwrap();
//                 let info = InitRoomInfo::new("test".to_string());
//                 let room_id = self.room_manager.create_room(info);

//                 let (recv_msg_chan_tx,recv_msg_chan_rx) = channel();
//                 let join_info = JoinRoomInfo::new(player_id, recv_msg_chan_rx);
//                 self.room_manager.join_room(room_id, join_info);
//                 self.players.insert(player_id, recv_msg_chan_tx.clone());
//                 self.conn_mgr.set_recv_message_chan(conn_id, recv_msg_chan_tx);
//             }
//             if msg.params()[0] == "join_room" {
//                 println!("join_room");
//                 let player_id = msg.params()[1].parse::<PlayerID>().unwrap();
//                 let room_id = msg.params()[2].parse::<u32>().unwrap();
//                 let (recv_msg_chan_tx,recv_msg_chan_rx) = channel();

//                 let info = JoinRoomInfo::new(player_id, recv_msg_chan_rx);
//                 self.room_manager.join_room(room_id, info);
//                 self.players.insert(player_id, recv_msg_chan_tx.clone());
//                 self.conn_mgr.set_recv_message_chan(conn_id, recv_msg_chan_tx);
//             }
//         }
    }
}

pub fn sample1_start() {
    let mut game = Sample1Game::new();
    loop {
        game.update();
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    sample1_start();
}
