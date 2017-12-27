extern crate ragtime;

use std::sync::mpsc::*;
use ragtime::network;
use ragtime::string_message;
use protocol::Protocol;

pub struct Input {
//    msg_chan_rx: MsgChanRx<StringMessage>,
}

impl Input {
    pub fn new() -> Input {
        Input {
//            msg_chan_rx: rx,
        }
    }
    pub fn update(&mut self, protocols: &mut Vec<Protocol>) {
        //recv from chan
    }
}
