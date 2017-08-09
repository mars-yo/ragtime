pub mod input;
pub mod position;

pub mod typedef {
    use ragtime::game_object_manager::ComponentType;
    pub static Input:ComponentType = 1;
    pub static Position:ComponentType = 2;
}
