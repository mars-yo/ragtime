extern crate ragtime;

use std::sync::mpsc::Receiver;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct Input {
    msg_chan_rx: Receiver<MessageOnChannel<StringMessage>>,
    pos: (f32,f32),
}

impl Input {
    pub fn new(rx: Receiver<MessageOnChannel<StringMessage>>) -> Input {
        Input {
            msg_chan_rx: rx,
            pos: (0f32,0f32),
        }
    }
    pub fn update(&mut self) {
        if let Ok(msg) = self.msg_chan_rx.try_recv() {
            println!("room msg {}", msg.1.params()[0]);
        }
    }
    pub fn pos(&self) -> (f32,f32) {
        self.pos
    }
}
