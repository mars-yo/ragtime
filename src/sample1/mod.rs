use entity_component::component::*;
use entity_component::entity::*;
use entity_component::system::*;
use sample1::sub_component::position::*;
use super::sub_component::game_logic::connection_manager::*;
use super::sub_component::game_logic::db_manager::*;
use super::sub_component::game_logic::game_object_manager::*;
use super::string_message::StringMessage;

mod sub_component;

type Sample1ConnectionManager = ConnectionManager<StringMessage, GameObjectComponent>;
type Sample1GameObjectManager = GameObjectManager<GameObjectComponent>;

declare_component!(GameLogicComponent = Sample1ConnectionManager, DBManager, Sample1GameObjectManager);
declare_component!(GameObjectComponent = Position);

impl MessageHandler<StringMessage> for GameObjectComponent {
    fn on_message(&mut self, id:ConnectionID, msg:&StringMessage ) {

    }
}

struct Sample1Game {
    system:System<GameLogicComponent>,
}

pub fn sample1_start() {
    let mut system = Sample1Game{system:System::<GameLogicComponent>::new()};
    let conn = GameLogicComponent::Sample1ConnectionManager(Sample1ConnectionManager::new("127.0.0.1:8080".to_string()));
    let conn = Component::new( system.system.generate_entity_id(), conn);


    //make game AsMut
    //add connection manager
    //add game object manager

}
