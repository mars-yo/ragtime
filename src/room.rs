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
    fn update(&mut self, commands: Vec<Self::CommandType>);
    fn deletable(&self) -> bool;
}

pub fn start_thread<R>() -> Sender<(RoomID,R::CommandType)> where R:Room {

    let (tx,rx) = channel();
    thread::spawn(move||{
        struct RoomEntry<T> {
            room_id: RoomID,
            room: T,
        }
        let mut rooms = Vec::<RoomEntry<R>>::new();
        loop {
            let mut commands = HashMap::<RoomID,Vec<R::CommandType>>::new();
            loop {
                if let Ok(cmd) = rx.try_recv() {
                    let (room_id, cmd) = cmd;
                    let cmds = commands.entry(room_id).or_insert(Vec::new());
                    cmds.push(cmd);
                } else {
                    break;
                }
            }
            
            rooms.iter_mut().for_each(|r| {
                let mut cmds = commands.remove(&r.room_id);
                if cmds.is_none() {
                    cmds = Some(Vec::new());
                }
                cmds.map( |c| { r.room.update(c); } );
            });
            
            for cmd in commands {
                let (room_id, cmd) = cmd;
                let mut new_room = RoomEntry{ room_id: room_id, room: R::new(room_id) };
                new_room.room.update(cmd);
                rooms.push( new_room );
            }
            
            fn alive<R:Room>(r: &RoomEntry<R>) -> bool {
                !r.room.deletable()
            }
            rooms.retain( &alive::<R> );
            thread::sleep(Duration::from_millis(1000));
        }
    });
    tx
}
