use std::cmp::PartialEq;
use std::time::Duration;
use std::borrow::Borrow;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use std::collections::HashMap;

//create room when id not found.
//command type as asssiated.
// continuance -> can_delete

pub type RoomID = u32;

pub trait Room {
    type CommandType: Send + 'static;
    fn new(id:RoomID) -> Self;
    fn on_command(&mut self, cmd:Self::CommandType);
    fn update(&mut self);
    fn deletable(&self) -> bool;
}

struct RoomsInThread<R:Room> {
    chan_tx:Sender<(RoomID,R::CommandType)>,
}

impl<R> RoomsInThread<R> where R:Room {

    fn new() -> RoomsInThread<R> {
        // TODO immediately start thread.
        let (tx,rx) = channel();
        RoomsInThread {
            chan_tx:tx,
        }
    }
    fn send_command(&mut self, id:RoomID, cmd:R::CommandType ) {
        self.chan_tx.send((id,cmd));
    }
    fn start(&mut self) {
        // let (tx,rx):(Sender<Self::CommandType>,Receiver<Self::CommandType>) = channel();
        let (tx,rx) = channel();
        self.chan_tx = tx;
        thread::spawn(move||{
            let mut rooms = Vec::<(RoomID,R)>::new();
            loop {
                if let Ok(cmd) = rx.try_recv() {
                    let (room_id, cmd) = cmd;
                    let mut found = false;
                    {
                        let room = rooms.iter().find(|r| r.0 == room_id);
                        if room.is_some() {
                            found = true;
                        }
                    }
                    if !found {
                        let new_room = (room_id, R::new(room_id));
                        rooms.push( new_room );
                    }
                    let room = rooms.iter_mut().find(|r| r.0 == room_id).unwrap();
                    room.1.on_command(cmd);
//                     let mut found = false;
//                     for r in rooms.iter_mut() {
//                         let (room_id, ref mut room) = *r;
//                         if room_id == id {
//                             room.on_command(cmd);
//                             found = true;
//                             break;
//                         }
//                     }
//                     if !found {
//                         let mut room = R::new(id);
//                         room.on_command(cmd);
//                         rooms.push((id,room));
//                     }
                }
                
                fn update<R:Room>(r: &mut (RoomID,R)) {
                    r.1.update();
                }
                rooms.iter_mut().for_each(&update::<R>);
                
                fn alive<R:Room>(r: &(RoomID,R)) -> bool {
                    !r.1.deletable()
                }
                rooms.retain( &alive::<R> );
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }
}

type RoomThreadID = usize;

pub struct RoomManager<R:Room> {
    num_thread:i32,
    next_entry:RoomThreadID,
    room_id_map:HashMap<RoomID, RoomThreadID>,//roomidとthreadをひもづける必要あり
    rooms:Vec<RoomsInThread<R>>,
    next_room_id:RoomID,
}

impl<R> RoomManager<R> where R:Room {
    pub fn new(num_thread:i32) -> RoomManager<R> {
        let mut rooms = Vec::new();
        for i in 0..num_thread {
            let mut room = RoomsInThread::<R>::new();
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
    pub fn create_room(&mut self) -> RoomID {
        if self.next_room_id >= RoomID::max_value() {
            panic!("room id max");
        }
        let ret = self.next_room_id;
        self.room_id_map.insert(self.next_room_id, self.next_entry);
        self.next_room_id += 1;
        self.next_entry = (self.next_entry + 1) % self.rooms.len();
        ret
    }
    pub fn send_cmd(&mut self, room_id:RoomID, cmd:R::CommandType) {
        if let Some(thread_id) = self.room_id_map.get(&room_id) {
            self.rooms[*thread_id].send_command(room_id, cmd);
        } else {
            println!("room id {} not found", room_id);
        }
    }
    pub fn find_room(&self) {}
}
