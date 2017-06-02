#[macro_use]
extern crate ragtime;

mod input;
mod position;
mod receptor;
mod room;

use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use std::sync::mpsc::{channel,Receiver,Sender};
use input::*;
use receptor::*;
use position::*;
use room::*;

use ragtime::entity_component::component::*;
use ragtime::entity_component::entity::*;
use ragtime::entity_component::system::*;
use ragtime::connection_manager::*;
use ragtime::db_manager::DBManager;
use ragtime::game_object_manager::*;
use ragtime::room_manager::*;
use ragtime::string_message::*;

struct Sample1Game {
    room_manager:RoomManager<Sample1Room>,
    connection_manager: ConnectionManager<StringMessage>,
    db_manager: DBManager,
    players: HashMap<PlayerID, Sender<MessageOnChannel<StringMessage>>>,
    receptor_recv_msg_chan_rx: Receiver<MessageOnChannel<StringMessage>>,
}

impl Sample1Game {
    fn new() -> Sample1Game {

        let (tx,rx) = channel();

        let mut conn = ConnectionManager::<StringMessage>::new("127.0.0.1:8080".to_string(), tx);

        let room_man = RoomManager::<Sample1Room>::new(2);
        let db = DBManager::new();

        Sample1Game {
            room_manager: room_man,
            connection_manager: conn,
            db_manager: db,
            players: HashMap::new(),
            receptor_recv_msg_chan_rx: rx,
        }
    }
    fn update(&mut self) {
        //recv msg from chann, create room when requested, join room when requested,
        self.connection_manager.update();

        if let Ok(msg) = self.receptor_recv_msg_chan_rx.try_recv() {
            let conn_id = msg.0;
            let msg = msg.1;
            if msg.params()[0] == "create_room" {
                println!("create_room");
                let player_id = msg.params()[1].parse::<PlayerID>().unwrap();
                let info = InitRoomInfo::new("test".to_string());
                let room_id = self.room_manager.create_room(info);

                let (recv_msg_chan_tx,recv_msg_chan_rx) = channel();
                let join_info = JoinRoomInfo::new(player_id, recv_msg_chan_rx);
                self.room_manager.join_room(room_id, join_info);
                self.players.insert(player_id, recv_msg_chan_tx.clone());
                self.connection_manager.set_recv_message_chan(conn_id, recv_msg_chan_tx);
            }
            if msg.params()[0] == "join_room" {
                println!("join_room");
                let player_id = msg.params()[1].parse::<PlayerID>().unwrap();
                let room_id = msg.params()[2].parse::<u32>().unwrap();
                let (recv_msg_chan_tx,recv_msg_chan_rx) = channel();

                let info = room::JoinRoomInfo::new(player_id, recv_msg_chan_rx);
                self.room_manager.join_room(room_id, info);
                self.players.insert(player_id, recv_msg_chan_tx.clone());
                self.connection_manager.set_recv_message_chan(conn_id, recv_msg_chan_tx);
            }
        }
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
