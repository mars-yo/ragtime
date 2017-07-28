use std::collections::{HashMap,HashSet,BTreeSet};
use std::rc::Rc;
use std::cell::RefCell;
use id_generator::IDGenerator;

type ObjectID = i32;
type ComponentID = i32;

pub trait GameObject {
    fn id(&self) -> ObjectID;
    fn set_id(&mut self, id:ObjectID);
    fn update_component(&mut self, comp_id:ComponentID);
    fn is_zombie(&self) -> bool;
}

pub struct GameObjectManager
{
    id_gen: IDGenerator,
    objects: HashMap<ObjectID, Rc<RefCell<GameObject>>>,
    component_ids: BTreeSet<ComponentID>,
}

impl GameObjectManager {
    fn add_object(&mut self, obj:Rc<RefCell<GameObject>>) {
        let id = self.id_gen.next();
        {
            let mut obj = obj.borrow_mut();
            obj.set_id(id);
        }
        self.objects.insert(id, obj);
    }
    fn set_component_id(&mut self, ids:&mut [ComponentID]) {
        for id in ids {
            self.component_ids.insert(*id);
        }
    }
    fn update(&mut self) {
        let mut to_remove = HashSet::new();
        for cid in &self.component_ids {
            for (_, o) in &self.objects {
                match o.try_borrow_mut() {
                    Ok(mut o) => {
                        if o.is_zombie() {
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
