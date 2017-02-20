//extern crate rand;
use std::rc::Rc;
use std::rc::Weak;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub type SubComponentType = i32;
pub type EntityID = i32;

pub trait SubComponent {
    fn start(&mut self);
    fn update(&mut self);
}

type Entity<T> = HashMap<SubComponentType, Weak<Component<T>>>;

pub struct Component<T:SubComponent> {
    entity_id:EntityID,
    first_call:bool,
    sub_component:T,
}

impl<T> Component<T> where T:SubComponent {
    pub fn entity_id(&self) -> EntityID {
        self.entity_id
    }
    pub fn sub_component(&self) -> &T {
        &self.sub_component
    }
    fn start(&mut self) {
        self.sub_component.start();
    }
    fn update(&mut self) {
        if self.first_call {
            self.sub_component.start();
            self.first_call = false;
        }
        self.sub_component.update();
    }
}

pub struct System<T:SubComponent> {
    components:BTreeMap<SubComponentType,Vec<Rc<Component<T>>>>,
    entities:HashMap<EntityID,Entity<T>>,
}

impl<T> System<T> where T:SubComponent {
    pub fn update(&mut self) {
        for (_,comps) in self.components.iter_mut() {
            for c in comps.iter_mut() {
                if let Some(c) = Rc::get_mut(c) {
                    c.update();
                }
            }
        }
    }

    pub fn add_component(&mut self, comp_type:SubComponentType, c:Component<T>) -> Weak<Component<T>> {
        let entity_id = c.entity_id();
        let c = Rc::new(c);
        let weakc = Rc::downgrade(&c);
        let ret = Rc::downgrade(&c);

        // add component
        if ! self.components.contains_key(&comp_type) {
            let v = Vec::new();
            self.components.insert(comp_type, v);
        }
        self.components.get_mut(&comp_type).unwrap().push(c);

        // add entity
        if ! self.entities.contains_key(&entity_id) {
            let m = HashMap::new();
            self.entities.insert(entity_id, m);
        }
        let ent = self.entities.get_mut(&entity_id).unwrap();
        if ent.contains_key(&comp_type) {
            panic!("entity id {} already includes component type {}", entity_id, comp_type);
        }
        ent.insert(comp_type, weakc);

        ret
    }
}
