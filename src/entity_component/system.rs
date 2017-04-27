use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use entity_component::component::*;
use entity_component::entity::EntityID;
use entity_component::entity::Entity;

pub struct System {
    next_entity_id: i32,
    components: BTreeMap<UpdateOrder, Vec<Rc<RefCell<Component>>>>,
    entities: HashMap<EntityID, Entity>,
}

impl System {
    pub fn new() -> System {
        System {
            next_entity_id: 0,
            components: BTreeMap::new(),
            entities: HashMap::new(),
        }
    }

    fn generate_entity_id(&mut self) -> EntityID {
        if self.next_entity_id == i32::max_value() {
            panic!("entity id max");
        }
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    pub fn update(&self) {
        for (_, comps) in self.components.iter() {
            for c in comps.iter() {
                if let Ok(ref mut c) = c.try_borrow_mut() {
                    c.update();
                } else {
                    println!("can not borrow mut");
                }
            }
        }
    }

    pub fn new_entity(&mut self) -> &mut Entity {
        let ent = Entity::new( self.generate_entity_id() );
        let id = ent.id();
        if !self.entities.contains_key(&id) {
            self.entities.insert(id, ent);
        }
        self.entities.get_mut(&id).unwrap()
    }

    // pub fn add_component(&mut self,
    //                      order: UpdateOrder,
    //                      c: Component<T>)
    //                      -> Weak<RefCell<Component<T>>> {
    //     let entity_id = c.entity_id();
    //     let c = Rc::new(RefCell::new(c));
    //     let weakc = Rc::downgrade(&c);
    //     let ret = Rc::downgrade(&c);
    //
    //     // add component
    //     if !self.components.contains_key(&order) {
    //         let v = Vec::new();
    //         self.components.insert(order, v);
    //     }
    //     self.components.get_mut(&order).unwrap().push(c);
    //
    //     // add entity
    //     if !self.entities.contains_key(&entity_id) {
    //         let mut m = Entity::<T>::new(entity_id);
    //         m.add_component(order, weakc);
    //         self.entities.insert(entity_id, m);
    //     }
    //
    //     // let ent = self.entities.get_mut(&entity_id).unwrap();
    //     // ent.insert(order, weakc);
    //
    //     ret
    // }
}
