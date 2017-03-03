// use entity_component::*;
// use self::position::Position;
// use self::connection_manager::ConnectionManager;
// use entity_component::component::*;
//
// pub mod position;
// pub mod connection_manager;
//
// pub enum SubComponents {
//     ConnectionManagerSC(ConnectionManager),
//     PositionSC(Position),
// }
//
// impl SubComponent for SubComponents {
//     fn update(&mut self) {
//         match *self {
//             SubComponents::ConnectionManagerSC(ref mut c) => {
//                 c.update();
//             },
//             SubComponents::PositionSC(ref mut c) => {
//                 c.update();
//             }
//         }
//     }
//     fn start(&mut self) {
//         match *self {
//             SubComponents::ConnectionManagerSC(ref mut c) => {
//                 c.start();
//             },
//             SubComponents::PositionSC(ref mut c) => {
//                 c.start();
//             }
//         }
//     }
// }
