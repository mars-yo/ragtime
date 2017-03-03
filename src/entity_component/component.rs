use entity_component::entity::EntityID;

pub type UpdateOrder = i32;

pub trait SubComponent {
    fn start(&mut self);
    fn update(&mut self);
}

pub struct Component<T:SubComponent> {
    entity_id:EntityID,
    first_call:bool,
    sub_component:T,
}

impl<T> Component<T> where T:SubComponent {

    pub fn new(id:EntityID, sub:T) -> Self {
        Self {
            entity_id:id,
            first_call:false,
            sub_component:sub,
        }
    }

    pub fn entity_id(&self) -> EntityID {
        self.entity_id
    }
    pub fn sub_component(&self) -> &T {
        &self.sub_component
    }
    pub fn update(&mut self) {
        if self.first_call {
            self.sub_component.start();
            self.first_call = false;
        }
        self.sub_component.update();
    }
}
