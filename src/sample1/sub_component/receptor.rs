use connection_manager::*;
use string_message::*;
use game_object_manager::*;

//新規接続用
//新規CLが来たら、プレイヤーオブジェクトを生成、GameObjectManager,ConnectionManagerに登録する

pub struct Receptor {}

impl Receptor {
    pub fn new() -> Receptor {
        Receptor {}
    }
    pub fn start(&mut self) {}
    pub fn update(&mut self) {}
    pub fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {
        let params = msg.params();
        if params.len() <= 0 {
            return;
        }
        if let (Some(cmd), Some(arg)) = (params.get(0), params.get(1)) {
            if *cmd == "login" {
                println!("login {}", *arg);
            }

        }
    }
}
