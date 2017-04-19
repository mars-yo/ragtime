use entity_component::component::*;
use entity_component::entity::*;
use entity_component::system::*;

pub struct GameObjectManager<T>
    where T: SubComponent
{
    system: System<T>,
}

impl<T> SubComponent for GameObjectManager<T>
    where T: SubComponent
{
    fn start(&mut self) {}
    fn update(&mut self) {
        self.system.update();
    }
}
