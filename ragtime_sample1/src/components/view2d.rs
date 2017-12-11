extern crate ragtime;

use ragtime::network;
use ragtime::string_message;
use super::super::*;

pub struct View2D {
    msg_tx: MsgTx,
}

impl View2D {
    pub fn new(tx: MsgTx) -> View2D {
        View2D {
            msg_tx: tx,
        }
    }
    pub fn update(&mut self) {
    }
}
