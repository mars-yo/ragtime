use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use entity_component::component::*;

pub type EntityID = i32;

pub struct Entity
{
    id: EntityID,
    components: HashMap<UpdateOrder, Rc<RefCell<Component>>>,
    deleted: bool,
}

impl Entity
{
    pub fn new(id: EntityID) -> Entity {
        Entity {
            id: id,
            components: HashMap::new(),
            deleted: false,
        }
    }
    pub fn id(&self) -> EntityID {
        self.id
    }
    pub fn add_component<'a>(&'a mut self,
                             order: UpdateOrder,
                             c: Rc<RefCell<Component>>)
                             -> &'a mut Self {
        self.components.insert(order, c);
        self
    }
    pub fn set_deleted(&mut self) {
        self.deleted = true;
    }
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}
