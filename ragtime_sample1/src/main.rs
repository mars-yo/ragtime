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
use std::sync::mpsc::{channel,Receiver};
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


// type Sample1ConnectionManager = ConnectionManager<StringMessage, Component<ObjectComponent>>;
// type Sample1GameObjectManager = GameObjectManager<ObjectComponent>;
//
// declare_component!(LogicComponent = Sample1GameObjectManager);
// declare_component!(ObjectComponent = Receptor, Input, Position);
//
// impl MessageHandler<StringMessage> for Component<ObjectComponent> {
//     fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {
//         match *self.sub_component_mut() {
//             ObjectComponent::Receptor(ref mut cmp) => {
//                 cmp.on_message(id, msg);
//             }
//             ObjectComponent::Input(ref mut cmp) => {
//                 cmp.on_message(id, msg);
//             }
//             _ => {}
//         }
//     }
// }

struct Sample1Game {
    room_manager:RoomManager<Sample1Room>,
    connection_manager: ConnectionManager<StringMessage>,
    db_manager: DBManager,
    recv_msg_chan_rx: Receiver<MessageOnChannel<StringMessage>>,
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
            recv_msg_chan_rx: rx,
        }
    }
    fn update(&mut self) {
        //recv msg from chann, create room when requested, join room when requested,
        self.connection_manager.update();
        
        if let Ok(msg) = self.recv_msg_chan_rx.try_recv() {
            let conn_id = msg.0;
            let msg = msg.1;
            if msg.params()[0] == "create_room" {
                println!("create_room");
                let (recv_msg_chan_tx,recv_msg_chan_rx) = channel();
                let info = room::Sample1RoomInitializeInfo::new(recv_msg_chan_rx);
                self.room_manager.create_room(info);
                self.connection_manager.set_message_handler(conn_id, recv_msg_chan_tx);
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
