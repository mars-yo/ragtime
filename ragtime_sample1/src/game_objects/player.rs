use ragtime::game_object_manager::*;


struct Player {
    input: Input,
    pos: Position,
}

impl GameObject for Player {
    fn id(&self) -> ObjectID {

    }
    fn set_id(&mut self, id:ObjectID) {

    }
    fn update_component(&mut self, comp_id:ComponentID) {

    }
    fn is_zombie(&self) -> bool {

    }

}
