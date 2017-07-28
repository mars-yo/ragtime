extern crate ragtime;

use std::sync::mpsc::Receiver;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct Input {
    msg_chan_rx: Receiver<MessageOnChannel<StringMessage>>,
    pos: (f32,f32),
}

impl Input {
    fn start(&mut self) {}
    fn update(&mut self) {
        //recv from chan
    }
    fn pos(&self) -> (f32,f32) {
        self.pos
    }
}
