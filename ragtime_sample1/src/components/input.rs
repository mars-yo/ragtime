extern crate ragtime;

use std::sync::mpsc::Receiver;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct Input {
    msg_chan_rx: Receiver<MessageOnChannel<StringMessage>>,
    moving: (f32,f32),
}

impl Input {
    pub fn new(rx: Receiver<MessageOnChannel<StringMessage>>) -> Input {
        Input {
            msg_chan_rx: rx,
            moving: (0f32,0f32),
        }
    }
    pub fn update(&mut self) {
        self.moving.0 = 0f32;
        self.moving.1 = 0f32;
        if let Ok(msg) = self.msg_chan_rx.try_recv() {
            println!("room msg {}", msg.1.params()[0]);
            if msg.1.params()[0] == "move" {
                self.moving.0 = msg.1.params()[1].parse::<f32>().unwrap();
                self.moving.1 = msg.1.params()[2].parse::<f32>().unwrap();
            }
        }
    }
    pub fn moving(&self) -> (f32,f32) {
        self.moving
    }
}
