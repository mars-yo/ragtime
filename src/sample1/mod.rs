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
use connection_manager::ConnectionID;
use connection_manager::*;
use db_manager::*;
use game_object_manager::*;
use super::string_message::*;


type Sample1ConnectionManager = ConnectionManager<StringMessage, Component<ObjectComponent>>;
type Sample1GameObjectManager = GameObjectManager<ObjectComponent>;

declare_component!(LogicComponent = Sample1GameObjectManager);
declare_component!(ObjectComponent = Receptor, Input, Position);

impl MessageHandler<StringMessage> for Component<ObjectComponent> {
    fn on_message(&mut self, id: ConnectionID, msg: &StringMessage) {
        match *self.sub_component_mut() {
            ObjectComponent::Receptor(ref mut cmp) => {
                cmp.on_message(id, msg);
            }
            ObjectComponent::Input(ref mut cmp) => {
                cmp.on_message(id, msg);
            }
            _ => {}
        }
    }
}

struct Sample1Game {
    rooms: Vec<System<LogicComponent>>,
    connection_manager: Rc<RefCell<Sample1ConnectionManager>>,
    db_manager: Rc<RefCell<DBManager>>,
    receptor: Rc<RefCell<Component<ObjectComponent>>>,
}

impl Sample1Game {
    fn new() -> Sample1Game {

        let receptor = ObjectComponent::Receptor(Receptor::new());
        let receptor = Component::new(0, receptor);
        let receptor = Rc::new(RefCell::new(receptor));

        let mut conn = Sample1ConnectionManager::new("127.0.0.1:8080".to_string());
        conn.set_default_message_handler(Rc::downgrade(&receptor));

        let mut db = Rc::new(RefCell::new(DBManager::new()));

        Sample1Game {
            rooms: Vec::new(),
            connection_manager: Rc::new(RefCell::new(conn)),
            db_manager: db,
            receptor: receptor,
        }
    }
    fn update(&mut self) {
        self.connection_manager.borrow_mut().update();
        self.db_manager.borrow_mut().update();
        for room in self.rooms.iter_mut() {
            room.update();
        }
    }
}

pub fn sample1_start() {
    let mut game = Sample1Game::new();
    loop {
        game.update();
        thread::sleep(Duration::from_millis(1000));
    }

}
