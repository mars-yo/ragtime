#[macro_use]
extern crate log;

use entity_component::component::SubComponent;
use std::{thread, time};

mod entity_component;
mod sub_component;
mod sample1;


//connection_managerはゲーム、ルーム共通にしたい

fn main() {
    let mut conn_manager = sub_component::game_logic::connection_manager::ConnectionManager::new("127.0.0.1:53000".to_string());

}

#[test]
fn test() {
    let mut conn_manager = sub_component::game_logic::connection_manager::ConnectionManager::new("0.0.0.0:53000".to_string());
    conn_manager.start();
    loop {
        println!("loop");
        conn_manager.update();
        thread::sleep(time::Duration::from_secs(1));
    }
}
