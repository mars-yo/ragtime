use std::num;

pub struct IDGenerator {
    next_id:i32,
}

impl IDGenerator {

    pub fn new() -> IDGenerator {
        IDGenerator {
            next_id:0,
        }
    }

    pub fn next(&mut self) -> i32 {
        if self.next_id == i32::max_value() {
            panic!("reached max value");
        }
        let id = self.next_id;
        self.next_id = self.next_id + 1;
        id
    }
}
