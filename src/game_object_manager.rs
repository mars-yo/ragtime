use entity_component::component::*;
use entity_component::entity::*;
use entity_component::system::*;

pub struct GameObjectManager
{
    system: System,
}

impl Component for GameObjectManager
{
    fn start(&mut self) {}
    fn update(&mut self) {
        self.system.update();
    }
}
