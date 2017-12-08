extern crate ragtime;

use ragtime::connection_manager::*;
use ragtime::string_message::*;

pub struct View2D {
    msg_chan_tx: MsgChanTx<StringMessage>,
}

impl View2D {
    pub fn new(msg_chan_tx: MsgChanTx<StringMessage>) -> View2D {
        View2D {
          msg_chan_tx: msg_chan_tx,
        }
    }
    pub fn update(&mut self) {
    }
}
