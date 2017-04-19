mod sub_component;

use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::time::Duration;
use std::thread;
use entity_component::component::*;
use entity_component::entity::*;
use entity_component::system::*;
use sample1::sub_component::position::*;
use sample1::sub_component::input::*;
use sample1::sub_component::receptor::*;
use super::sub_component::game_logic::connection_manager::*;
use super::sub_component::game_logic::db_manager::*;
use super::sub_component::game_logic::game_object_manager::*;
use super::string_message::*;


type Sample1ConnectionManager = ConnectionManager<StringMessage, Component<GameLogicComponent>>;
type Sample1GameObjectManager = GameObjectManager<GameObjectComponent>;

declare_component!(GameLogicComponent = Sample1ConnectionManager,
                   DBManager,
                   Sample1GameObjectManager,
                   Receptor);
declare_component!(GameObjectComponent = Input, Position);

impl MessageHandler<StringMessage> for Component<GameLogicComponent> {
    fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {
        match *self.sub_component_mut() {
            GameLogicComponent::Receptor(ref mut cmp) => {
                cmp.on_message(id, msg);
            }
            _ => {}
        }
    }
}

impl MessageHandler<StringMessage> for Component<GameObjectComponent> {
    fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {
        match *self.sub_component_mut() {
            GameObjectComponent::Input(ref mut cmp) => {
                cmp.on_message(id, msg);
            }
            _ => {}
        }
    }
}

struct Sample1Game {
    system: System<GameLogicComponent>,
    connection_manager: Rc<RefCell<Component<GameLogicComponent>>>,
    receptor: Rc<RefCell<Component<GameLogicComponent>>>,
}

impl Sample1Game {
    fn new() -> Sample1Game {

        let mut system = System::<GameLogicComponent>::new();

        let singleton_id = system.generate_entity_id();

        let receptor = GameLogicComponent::Receptor(Receptor::new());
        let receptor = Component::new(singleton_id, receptor);
        let receptor = system.add_component(1, receptor);

        let mut conn = Sample1ConnectionManager::new("127.0.0.1:8080".to_string());
        conn.set_default_message_handler(receptor.clone());
        let conn = GameLogicComponent::Sample1ConnectionManager(conn);
        let conn = Component::new(singleton_id, conn);
        let conn = system.add_component(2, conn);

        Sample1Game {
            system: System::<GameLogicComponent>::new(),
            connection_manager: conn.upgrade().unwrap(),
            receptor: receptor.upgrade().unwrap(),
        }
    }
    fn update(&mut self) {
        self.system.update();
    }
}

pub fn sample1_start() {
    let mut game = Sample1Game::new();
    loop {
        game.update();
        thread::sleep(Duration::from_millis(1000));
    }

}
