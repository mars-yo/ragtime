pub mod input;
pub mod position;

pub mod typedef {
    use ragtime::game_object_manager::ComponentType;
    pub const Input:ComponentType = 1;
    pub const Position:ComponentType = 2;
}
