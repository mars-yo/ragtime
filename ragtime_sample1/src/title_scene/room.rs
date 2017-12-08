extern crate ragtime;

use ragtime::connection_manager::*;

enum Command {
    Accept((ConnectionID,MsgChanTx<StringMessage>,MsgChanRx<StringMessage>)),
}

pub struct TitleRoom {
    type CommandType = Command;
    //object manager
    cxns: HashMap<ConnectionID, MsgChanTx<StringMessage>, MsgChanRx<StringMessage>>,
  
}
impl Room for TitleRoom {
    pub fn new() -> TittleRoom {
      
    }
    pub fn update(&mut self) {
        // recv initial protocol.
        // how to reconnect to game room?
    }
    pub fn on_command(&mut self, cmd: &Command) {
        // save tx,rx
    }
}

