#[macro_use]
extern crate ragtime;

mod components;
mod receptor;
mod game_scene;
mod protocol;

use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use std::sync::mpsc::{channel,Receiver,Sender};
use receptor::*;

//use title_scene::room::*;
use game_scene::room::*;
use protocol::*;

use ragtime::network::*;
use ragtime::db::*;
use ragtime::room::*;

type MsgTx = MsgChanTx<Protocol>;
type MsgRx = MsgChanRx<Protocol>;

struct Sample1Game {
//    title_room_mgr: RoomManager<TitleRoom>,
    game_room_mgr:RoomManager<Sample1Room>,
    conn_mgr: ConnectionManager<Protocol>,
    db_manager: DBManager,
}

impl Sample1Game {
    fn new() -> Sample1Game {

        let mut conn = ConnectionManager::<_>::new("127.0.0.1:8080".to_string());

        let grm = RoomManager::<Sample1Room>::new(2);
        let db = DBManager::new();

        Sample1Game {
//             title_room_mgr:
            game_room_mgr: grm,
            conn_mgr: conn,
            db_manager: db,
        }
    }
//     fn try_recv_data( cxt:&(ConnectionID,MsgChanTx,MsgChanRX)) -> bool {
//     }
    fn update(&mut self) {
        //recv msg from chann, create room when requested, join room when requested,
        if let Some(cxn) = self.conn_mgr.poll() {
//            self.new_cxns.push(cxn);
            // send cxn to title scene
            let room_id = self.game_room_mgr.create_room();
            let cmd = JoinCommand::new(1,cxn.1,cxn.2);
            self.game_room_mgr.send_cmd(room_id, RoomCommand::Join(cmd));
        }
      
//        self.new_cxns.retain(&Sample1Game::try_recv_data);
      
//         let new_msg = None:
//         for (conn_id, tx,rx)) in self.new_conn.iter() {
//             if let Ok(msg) = rx.try_recv() {
//                 // if valid command
//                 new_msg = msg;
//                 break;
//             }
//         }
      
            
      
//         if  let Some(msg) = new_msg {
//              let conn_id = msg.0;
//              let msg = msg.1;
//              if msg.params()[0] == "create_room" {
//                  println!("create_room");
//                  let player_id = msg.params()[1].parse::<PlayerID>().unwrap();
//                  let info = InitRoomInfo::new("test".to_string());
//                  let room_id = self.room_manager.create_room(info);
//                  let (tx,rx) = self.new_conn.remove(conn_id);

//                  let join_info = JoinRoomInfo::new(player_id, tx, rx);
//                  self.room_manager.join_room(room_id, join_info);
// //                 self.players.insert(player_id, recv_msg_chan_tx.clone());
// //                 self.conn_mgr.set_recv_message_chan(conn_id, recv_msg_chan_tx);
//              }
//              if msg.params()[0] == "join_room" {
//                  println!("join_room");
//                  let player_id = msg.params()[1].parse::<PlayerID>().unwrap();
//                  let room_id = msg.params()[2].parse::<u32>().unwrap();
//                  let info = JoinRoomInfo::new(player_id, tx, rx);
//                  self.room_manager.join_room(room_id, info);
// //                 self.players.insert(player_id, recv_msg_chan_tx.clone());
// //                 self.conn_mgr.set_recv_message_chan(conn_id, recv_msg_chan_tx);
//              }
          
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
//    }
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
