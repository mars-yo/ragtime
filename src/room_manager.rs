use entity_component::*;

//ルームにメッセージ受信用のチャンネルを持つのが良さそう

struct RoomUpdater {
    //thread
    //communication channel
    rooms:Vec<System<T>>,
}

pub struct RoomManager<T:SubComponent> {
    max_room_in_thread:i32,
}

impl RoomManager {
    pub fn new() -> RoomManager {}
    pub fn create_room(&mut self) {}
    pub fn find_room(&self) {}
}
