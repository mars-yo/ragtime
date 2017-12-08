extern crate ragtime;

use std::sync::mpsc::*;
use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct Input {
//    msg_chan_rx: MsgChanRx<StringMessage>,
}

impl Input {
    pub fn new() -> Input {
        Input {
//            msg_chan_rx: rx,
        }
    }
    pub fn start(&mut self) {}
    pub fn update(&mut self) {
        //recv from chan
    }
}
