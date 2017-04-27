use entity_component::entity::EntityID;

// #[macro_export]
// macro_rules! declare_component {
//     ($e:ident = $($c:ident),+) => {
//         enum $e {
//             $( $c($c), )+
//         }
//         impl SubComponent for $e {
//             fn start(&mut self) {
//                 match *self {
//                     $( $e::$c(ref mut cmp) => { cmp.start(); }, )+
//                 }
//             }
//             fn update(&mut self) {
//                 match *self {
//                     $( $e::$c(ref mut cmp) => { cmp.update(); }, )+
//                 }
//             }
//         }
//     }
// }

pub trait Component {
    fn start(&mut self);
    fn update(&mut self);
}

pub type UpdateOrder = i32;

pub struct ComponentBase {
    entity_id: EntityID,
    deleted: bool,
}

impl ComponentBase
{
    pub fn new(id: EntityID) -> ComponentBase {
        ComponentBase {
            entity_id: id,
            deleted: false,
        }
    }

    pub fn entity_id(&self) -> EntityID {
        self.entity_id
    }
}
