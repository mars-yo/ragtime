extern crate ragtime;
use std::collections::HashMap;
use ragtime::network;
use ragtime::string_message;
use super::*;

#[derive(Default)]
pub struct Receptor {
    cxns: HashMap<ConnectionID,(MsgTx,MsgRx)>,
}

impl Receptor {
    pub fn new() -> Receptor {
        Receptor {..Default::default()}
    }
}
impl Receptor {
    fn update(&mut self) {
        // recv msg from rx, do authentication
        // wait authentication, return roomtype, roomid, cxnid,tx,rx
    }
    fn add_entry(&mut self, id:ConnectionID, tx:MsgTx, rx:MsgRx) {
        self.cxns.insert(id, (tx,rx));
    }

}
// impl MessageHandler for Receptor {
//     pub fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {
//         let params = msg.params();
//         if params.len() <= 0 {
//             return;
//         }
//         if let (Some(cmd), Some(arg)) = (params.get(0), params.get(1)) {
//             if *cmd == "login" {
//                 println!("login {}", *arg);
//             }
//
//         }
//     }
// }
