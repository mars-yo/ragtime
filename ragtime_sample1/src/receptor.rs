extern crate ragtime;

use ragtime::connection_manager::*;
use ragtime::string_message::*;
use ragtime::game_object_manager::*;
use ragtime::entity_component::component::*;

//新規接続用
//新規CLが来たら、プレイヤーオブジェクトを生成、GameObjectManager,ConnectionManagerに登録する

pub struct Receptor {}

impl Receptor {
    pub fn new() -> Receptor {
        Receptor {}
    }
}
impl Component for Receptor {
    fn start(&mut self) {}
    fn update(&mut self) {}
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
