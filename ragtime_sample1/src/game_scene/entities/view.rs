use std::slice::Iter;

#[derive(Default)]
pub struct View {
    view2D: View2D,
    view3D: View3D,
}

impl View {
    pub fn new() -> View {
        View {..Default::default()}
    }
    fn update_view2D(&mut self) {
    }
    fn update_view3D(&mut self) {
    }
//     pub fn updaters() -> [fn(&mut Player);2] {
//         [Player::update_input, Player::update_position]
//     }

}
