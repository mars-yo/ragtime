use std::collections::{HashMap,HashSet,BTreeSet};
use std::rc::Rc;
use std::cell::RefCell;

type ObjectID = i32;
type ComponentID = i32;

pub trait GameObject {
    fn id(&self) -> ObjectID;
    fn component_ids(&self) -> &Vec<ComponentID>;
    fn update_component(&mut self, comp_id:ComponentID);
    fn is_deleted(&self) -> bool;
}

pub struct GameObjectManager
{
    objects: HashMap<ObjectID, Rc<RefCell<GameObject>>>,
    component_ids: BTreeSet<ComponentID>,
}

impl GameObjectManager {
    fn add_object(&mut self, obj:Rc<RefCell<GameObject>>) {
        let mut id:ObjectID = 0;
        {
            let obj = obj.borrow();
            id = obj.id();
            for cid in obj.component_ids() {
                self.component_ids.insert(*cid);
            }
        }
        self.objects.insert(id, obj);
    }
    fn update(&mut self) {
        let mut to_remove = HashSet::new();
        for cid in &self.component_ids {
            for (_, o) in &self.objects {
                match o.try_borrow_mut() {
                    Ok(mut o) => {
                        if o.is_deleted() {
                            to_remove.insert(o.id());
                        } else {
                            o.update_component(*cid);
                        }
                    },
                    Err(_) => {
                        error!("game object try_borrow_mut error");
                    }
                }
            }
        }
        for id in to_remove {
            self.objects.remove(&id);
        }
    }
}
