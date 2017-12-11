#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
pub mod network;
pub mod db;
pub mod string_message;
pub mod room;

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
