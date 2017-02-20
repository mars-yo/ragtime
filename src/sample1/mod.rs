use super::entity_component::{Component,SubComponent};
use std::rc::Weak;

enum SubComponents {
    A(AComponent),
    B(BComponent),
}

impl SubComponent for SubComponents {
    fn update(&mut self) {
        match *self {
            SubComponents::A(ref mut c) => {
                c.update();
            },
            SubComponents::B(ref mut c) => {
                c.update();
            }
        }
    }
    fn start(&mut self) {
        match *self {
            SubComponents::A(ref mut c) => {
                c.start();
            },
            SubComponents::B(ref mut c) => {
                c.start();
            }
        }
    }
}

struct BComponent {

}
impl SubComponent for BComponent {
    fn start(&mut self) {

    }
    fn update(&mut self) {

    }
}
impl BComponent {
    fn pos(&self) ->(f32,f32) {
        (0.0,0.0)
    }
}

struct AComponent {
    p:Weak<Component<BComponent>>
}

impl SubComponent for AComponent {
    fn start(&mut self) {

    }
    fn update(&mut self) {
        let p = self.p.upgrade().unwrap();
        let ps = p.sub_component().pos();
    }
}
