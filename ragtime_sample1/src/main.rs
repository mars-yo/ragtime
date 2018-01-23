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

type UserID = u64;

struct Sample1Game {
//    title_room_mgr: RoomManager<TitleRoom>,
    game_room_mgr:RoomManager<Sample1Room>,
    conn_mgr: ConnectionManager<Command>,
    db_manager: DBManager,
    user_mgr: Vec<(UserID,ConnectionID)>,
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
    fn update(&mut self) {
        if let Some(conn_id) = self.conn_mgr.listen() {
            
        }
        self.conn_mgr.recv(|conn_id,msg| {
            match msg {
                // if C2S command, resulve UserID and send to room
                
                Command::Login(c) => {
                    self.user_mgr.push( (conn_id, c.user_id) );
                },
                Command::Join(c) => {
                    // find conn id and get user_id
                    // create_room
                    // send cmd
                }
                _ => {},
            }
        });
        
//         if let Some(cxn) = self.conn_mgr.poll() {
//             let room_id = self.game_room_mgr.create_room();
//             let cmd = JoinCommand::new(1,cxn.1,cxn.2);
//             self.game_room_mgr.send_cmd(room_id, RoomCommand::Join(cmd));
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
