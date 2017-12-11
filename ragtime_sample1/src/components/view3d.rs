extern crate ragtime;

#[derive(Default)]
pub struct View3D {
}

impl View3D {
    pub fn new() -> View3D {
        View3D {
            ..Default::default()
        }
    }
    pub fn update(&mut self) {
    }
}
