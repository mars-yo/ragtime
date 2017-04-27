#[macro_use]
extern crate log;

#[macro_use]
pub mod entity_component;
pub mod connection_manager;
pub mod db_manager;
pub mod game_object_manager;
pub mod string_message;
pub mod room_manager;

//connection_managerはゲーム、ルーム共通にしたい

// fn main() {
//     //    let mut conn_manager = sub_component::game_logic::connection_manager::ConnectionManager::new("127.0.0.1:53000".to_string());
//     sample1::sample1_start();
// }

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
