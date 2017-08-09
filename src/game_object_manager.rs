use std::collections::{HashMap,HashSet,BTreeSet};
use std::rc::Rc;
use std::cell::RefCell;
use id_generator::IDGenerator;

pub type ObjectID = i32;
pub type ComponentType = i32;

pub trait GameObject {
    fn id(&self) -> ObjectID;
    fn set_id(&mut self, id:ObjectID);
    fn start(&mut self);
    fn update_component(&mut self, comp_id:ComponentType);
    fn is_zombie(&self) -> bool;
}

pub struct GameObjectManager
{
    id_gen: IDGenerator,
    objects: HashMap<ObjectID, Rc<RefCell<GameObject>>>,
    starting_obj_ids: HashSet<ObjectID>,
    component_types: BTreeSet<ComponentType>,
}

impl GameObjectManager {
    pub fn new() -> GameObjectManager {
        GameObjectManager {
            id_gen: IDGenerator::new(),
            objects: HashMap::new(),
            starting_obj_ids: HashSet::new(),
            component_types: BTreeSet::new(),
        }
    }
    pub fn add_object(&mut self, obj:Rc<RefCell<GameObject>>) {
        let id = self.id_gen.next();
        {
            let mut obj = obj.borrow_mut();
            obj.set_id(id);
        }
        self.objects.insert(id, obj);
        self.starting_obj_ids.insert(id);
        println!("obj added {}", id);
    }
    pub fn set_component_types(&mut self, ids:&[ComponentType]) {
        for id in ids {
            self.component_types.insert(*id);
        }
        println!("comp types {:?}", self.component_types);
    }
    pub fn update(&mut self) {
        for oid in &self.starting_obj_ids {
            let mut obj = self.objects.get_mut(&oid).unwrap().borrow_mut();
            obj.start();
        }
        self.starting_obj_ids.clear();

        let mut to_remove = HashSet::new();
        println!("comp types {:?}", self.component_types);
        for cid in &self.component_types {
            println!("updating component {}", cid);
            for (id, o) in &self.objects {
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
