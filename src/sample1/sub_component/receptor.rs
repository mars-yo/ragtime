use sub_component::game_logic::connection_manager::*;
use string_message::*;
use sub_component::game_logic::game_object_manager::*;

//新規接続用
//新規CLが来たら、プレイヤーオブジェクトを生成、GameObjectManager,ConnectionManagerに登録する

pub struct Receptor {}

impl Receptor {
    pub fn start(&mut self) {}
    pub fn update(&mut self) {}
    pub fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {}
}
