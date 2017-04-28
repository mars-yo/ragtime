use std::cmp::PartialEq;
use std::time::Duration;
use std::borrow::Borrow;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender,Receiver};
use entity_component::system::*;

#[derive(PartialEq)]
pub enum Continuance {
    Continue,
    Remove,
}

pub trait Room {
    type InitializeInfo: Send + 'static;
    fn new(info:Self::InitializeInfo) -> Self;
    fn update(&mut self) -> Continuance;
}

struct RoomsInThread<T:Room> {
    new_room_tx:Sender<T::InitializeInfo>,
}

impl<T> RoomsInThread<T> where T:Room {
    fn new() -> RoomsInThread<T> {
        let (tx,rx) = channel();
        RoomsInThread {
            new_room_tx:tx,
        }
    }
    fn add_room(&mut self, info:T::InitializeInfo) {
        self.new_room_tx.send(info);
    }
    fn start(&mut self) {
        let (new_room_tx,new_room_rx):(Sender<T::InitializeInfo>,Receiver<T::InitializeInfo>) = channel();
        self.new_room_tx= new_room_tx;
        thread::spawn(move||{
            let mut rooms = Vec::new();
            loop {
                if let Ok(new_room_info) = new_room_rx.try_recv() {
                    // let  new_room_info = new_room_info.borrow();
                    let room = T::new(new_room_info);
                    rooms.push(Box::new(room));
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

pub struct RoomManager<T:Room> {
    num_thread:i32,
    next_entry:i32,
    rooms:Vec<RoomsInThread<T>>,
}

impl<T> RoomManager<T> where T:Room {
    pub fn new(num_thread:i32) -> RoomManager<T> {
        let mut rooms = Vec::new();
        for i in 0..num_thread {
            let mut room = RoomsInThread::<T>::new();
            room.start();
            rooms.push(room);
        }
        RoomManager {
            num_thread: num_thread,
            next_entry:0,
            rooms:rooms,
        }
    }
    pub fn create_room(&mut self, info:T::InitializeInfo) {
        self.rooms[self.next_entry as usize].add_room(info);
        self.next_entry = (self.next_entry + 1) % self.rooms.len() as i32;
    }
    pub fn find_room(&self) {}
}
