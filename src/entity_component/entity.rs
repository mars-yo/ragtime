use std::collections::HashMap;
use std::rc::Weak;
use std::cell::RefCell;
use entity_component::component::*;

pub type EntityID = i32;

pub struct Entity<T>
    where T: SubComponent
{
    id: EntityID,
    components: HashMap<UpdateOrder, Weak<RefCell<Component<T>>>>,
}

impl<T> Entity<T>
    where T: SubComponent
{
    pub fn new(id: EntityID) -> Self {
        Entity::<T> {
            id: id,
            components: HashMap::new(),
        }
    }
    pub fn id(&self) -> EntityID {
        self.id
    }
    pub fn add_component<'a>(&'a mut self,
                             order: UpdateOrder,
                             c: Weak<RefCell<Component<T>>>)
                             -> &'a mut Self {
        self.components.insert(order, c);
        self
    }
}
