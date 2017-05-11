use std::cmp::PartialEq;
use std::time::Duration;
use std::borrow::Borrow;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use entity_component::system::*;
use std::collections::HashMap;

pub type RoomID = u32;

#[derive(PartialEq)]
pub enum Continuance {
    Continue,
    Remove,
}

pub enum RoomCommand<InitInfoType,JoinInfoType> {
    Create((RoomID,InitInfoType)),
    Join((RoomID,JoinInfoType)),
}

pub trait Room {
    type InitInfoType: Send + 'static;
    type JoinInfoType: Send + 'static;
    fn new(id:RoomID, info:Self::InitInfoType) -> Self;
    fn join(&mut self, info:Self::JoinInfoType);
    fn update(&mut self) -> Continuance;
    fn id(&self) -> RoomID;
}

struct RoomsInThread<RoomType> where RoomType:Room {
    chan_tx:Sender<RoomCommand<RoomType::InitInfoType,RoomType::JoinInfoType>>,
}

impl<RoomType> RoomsInThread<RoomType> where RoomType:Room {

    fn new() -> RoomsInThread<RoomType> {
        let (tx,rx) = channel();
        RoomsInThread {
            chan_tx:tx,
        }
    }
    fn add_room(&mut self, id:RoomID, info:RoomType::InitInfoType) {
        let cmd = RoomCommand::Create((id,info));
        self.chan_tx.send(cmd);
    }
    fn join_room(&mut self, id:RoomID, info:RoomType::JoinInfoType ) {
        let cmd = RoomCommand::Join((id,info));
        self.chan_tx.send(cmd);
    }
    fn start(&mut self) {
        // let (tx,rx):(Sender<Self::CommandType>,Receiver<Self::CommandType>) = channel();
        let (tx,rx) = channel();
        self.chan_tx = tx;
        thread::spawn(move||{
            let mut rooms = Vec::new();
            loop {
                if let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        RoomCommand::Create((id,info)) => {
                            let room = RoomType::new(id, info);
                            rooms.push(Box::new(room));
                        }
                        RoomCommand::Join((id,info)) => {
                            for r in rooms.iter_mut() {
                                if r.id() == id {
                                    r.join(info);
                                    break;
                                }
                            }
                        }
                    }
                    // let  new_room_info = new_room_info.borrow();
                }
                let mut to_remove = Vec::new();
                for i in 0..rooms.len() {
                    if rooms[i].update() == Continuance::Remove {
                        to_remove.push(i);
                    }
                }
                for index in &to_remove {
                    rooms.remove(*index);
                }
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }
}

type RoomThreadID = usize;

pub struct RoomManager<T:Room> {
    num_thread:i32,
    next_entry:RoomThreadID,
    room_id_map:HashMap<RoomID, RoomThreadID>,//roomidとthreadをひもづける必要あり
    rooms:Vec<RoomsInThread<T>>,
    next_room_id:RoomID,
}

impl<RoomType> RoomManager<RoomType> where RoomType:Room {
    pub fn new(num_thread:i32) -> RoomManager<RoomType> {
        let mut rooms = Vec::new();
        for i in 0..num_thread {
            let mut room = RoomsInThread::<RoomType>::new();
            room.start();
            rooms.push(room);
        }
        RoomManager {
            num_thread: num_thread,
            next_entry: 0,
            room_id_map: HashMap::new(),
            rooms: rooms,
            next_room_id: 0,
        }
    }
    pub fn create_room(&mut self, info:RoomType::InitInfoType) -> RoomID {
        if self.next_room_id >= RoomID::max_value() {
            panic!("room id max");
        }
        self.rooms[self.next_entry].add_room(self.next_room_id, info);
        let ret = self.next_room_id;
        self.room_id_map.insert(self.next_room_id, self.next_entry);
        self.next_room_id += 1;
        self.next_entry = (self.next_entry + 1) % self.rooms.len();
        ret
    }
    pub fn join_room(&mut self, room_id:RoomID, info:RoomType::JoinInfoType) {
        if let Some(thread_id) = self.room_id_map.get(&room_id) {
            self.rooms[*thread_id].join_room(room_id, info);
        }
    }
    pub fn find_room(&self) {}
}
