extern crate ragtime;

use std::sync::mpsc::Receiver;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct Input {
    msg_chan_rx: Receiver<MessageOnChannel<StringMessage>>,
    moving: (f32,f32),
}

impl Input {
    pub fn start(&mut self) {}
    pub fn update(&mut self) {
        //recv from chan
    }
    pub fn pos(&self) -> (f32,f32) {
        self.pos
    }
}
