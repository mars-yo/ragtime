use std::collections::HashMap;
use std::rc::Weak;
use entity_component::component::*;

pub type EntityID = i32;

pub struct Entity<T> where T:SubComponent {
    id:EntityID,
    components:HashMap<UpdateOrder, Weak<Component<T>>>,
}

impl<T> Entity<T> where T:SubComponent {
    pub fn new(id:EntityID) -> Self {
        Self {
            id:id,
            components:HashMap::new(),
        }
    }
    pub fn id(&self) -> EntityID {
        self.id
    }
    pub fn add_component<'a>(&'a mut self, order:UpdateOrder, c:Component<T>) -> &'a mut Self {

    }
}
