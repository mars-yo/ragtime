use std::rc::Rc;
use std::rc::Weak;
use std::collections::BTreeMap;
use std::collections::HashMap;
use entity_component::component::*;
use entity_component::entity::EntityID;
use entity_component::entity::Entity;

static next_entity_id : EntityID = 0;

fn generate_entity_id() -> EntityID {
    if next_entity_id == i32::max_value() {
        panic!("entity id max");
    }
    let id = next_entity_id;
    next_entity_id += 1;
    id
}

pub struct System<T:SubComponent> {
    components:BTreeMap<UpdateOrder,Vec<Rc<Component<T>>>>,
    entities:HashMap<EntityID,Entity<T>>,
}

impl<T> System<T> where T:SubComponent {

    pub fn new() -> System<T> {
        System::<T> {
            components:BTreeMap::new(),
            entities:HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        for (_,comps) in self.components.iter_mut() {
            for c in comps.iter_mut() {
                if let Some(c) = Rc::get_mut(c) {
                    c.update();
                }
            }
        }
    }

    pub fn add_component(&mut self, order:UpdateOrder, c:Component<T>) -> Weak<Component<T>> {
        let entity_id = c.entity_id();
        let c = Rc::new(c);
        let weakc = Rc::downgrade(&c);
        let ret = Rc::downgrade(&c);

        // add component
        if ! self.components.contains_key(&order) {
            let v = Vec::new();
            self.components.insert(order, v);
        }
        self.components.get_mut(&order).unwrap().push(c);

        // add entity
        if ! self.entities.contains_key(&entity_id) {
            let m = Entity::<T>::new(entity_id);
            self.entities.insert(entity_id, m);
        }
        let ent = self.entities.get_mut(&entity_id).unwrap();
        if ent.contains_key(&order) {
            panic!("entity id {} already includes update order {}", entity_id, order);
        }
        ent.insert(order, weakc);

        ret
    }
}
