use entity_component::entity::EntityID;

macro_rules! declare_component {
    ($e:ident = $($c:ident),+) => {
        enum $e {
            $( $c($c), )+
        }
        impl SubComponent for $e {
            fn start(&mut self) {
                match *self {
                    $( $e::$c(ref mut cmp) => { cmp.start(); }, )+
                }
            }
            fn update(&mut self) {
                match *self {
                    $( $e::$c(ref mut cmp) => { cmp.update(); }, )+
                }
            }
        }
    }
}

pub type UpdateOrder = i32;

pub trait SubComponent {
    fn start(&mut self);
    fn update(&mut self);
}

pub struct Component<T: SubComponent> {
    entity_id: EntityID,
    first_call: bool,
    sub_component: T,
}

impl<T> Component<T>
    where T: SubComponent
{
    pub fn new(id: EntityID, sub: T) -> Self {
        Component::<T> {
            entity_id: id,
            first_call: false,
            sub_component: sub,
        }
    }

    pub fn entity_id(&self) -> EntityID {
        self.entity_id
    }
    pub fn sub_component(&self) -> &T {
        &self.sub_component
    }
    pub fn update(&mut self) {
        if self.first_call {
            self.sub_component.start();
            self.first_call = false;
        }
        self.sub_component.update();
    }
}
