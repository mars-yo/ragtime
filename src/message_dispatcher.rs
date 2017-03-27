use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use sub_component::game_logic::connection_manager::ConnectionID;

pub trait MessageHandler {
    fn on_message( &mut self, msg:String );
}

pub struct MessageDispatcher<T:MessageHandler> {
    handlers:HashMap<ConnectionID, Weak<RefCell<T>>>,
}

impl<T:MessageHandler> MessageDispatcher<T> {
    pub fn new() -> MessageDispatcher<T> {
        MessageDispatcher {
            handlers:HashMap::new(),
        }
    }
    pub fn register( &mut self, conn_id:ConnectionID, obj:Weak<RefCell<T>> ) {
        self.handlers.insert(conn_id, obj);
    }
    pub fn dispatch( &mut self, conn_id:ConnectionID, msg:String ) {
        if let Some(mut obj) = self.handlers.get_mut(&conn_id) {
            if let Some(mut obj) = obj.upgrade() {
                if let Ok(ref mut obj) = obj.try_borrow_mut() {
                    obj.on_message(msg);
                }
            }
        }
    }
}
