#[macro_use]
extern crate log;

use entity_component::component::SubComponent;
use std::{thread, time};
use std::rc::{Rc,Weak};
use std::cell::RefCell;

mod entity_component;
mod sub_component;
mod sample1;
mod message_dispatcher;

//connection_managerはゲーム、ルーム共通にしたい

fn main() {
//    let mut conn_manager = sub_component::game_logic::connection_manager::ConnectionManager::new("127.0.0.1:53000".to_string());
    sample1::sample1_start();
}

struct Test {

}
impl Test {
    pub fn test(&self,id:i32,msg:String) {
        println!("recv on method{}", msg);
    }
}
impl message_dispatcher::MessageHandler for Test {
    fn on_message(&mut self, msg:String) {
        println!("recv --- {}", msg);
    }
}

#[test]
fn test2() {
    let t = Rc::new(RefCell::new(Test{}));
    let mut disp = message_dispatcher::MessageDispatcher::new();
    disp.register( 1, Rc::downgrade(&t) );
    disp.dispatch(1, "aaa".to_string());
}

#[test]
fn test() {
    // return;
    // let t = Test{};
    // let mut conn_manager = sub_component::game_logic::connection_manager::ConnectionManager::new("0.0.0.0:53000".to_string(), |id:i32,msg:String|{t.test(id,msg);});
    // conn_manager.start();
    // loop {
    //     let mut data:Vec<u8> = Vec::new();
    //     data.push(65u8);
    //     conn_manager.send_to(0, data);
    //     conn_manager.update();
    //     thread::sleep(time::Duration::from_secs(1));
    // }
}
