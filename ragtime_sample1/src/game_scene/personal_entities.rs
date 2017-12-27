use components::*;
use game_scene::entities::*;
use protocol::Protocol;

#[derive(Default)]
pub struct PersonalEntities {
//     uis: Vec<UI>,
//     cameras : Vec<Camera>,
}

impl PersonalEntities {
    pub fn new() -> PersonalEntities {
        PersonalEntities {..Default::default()}
    }
    pub fn update(&mut self, input: &Vec<Protocol>) {
        
//        let mut protocols = Vec::new();
//         self.uis.iter_mut().for_each( |o| o.update_input(&mut protocols) );
//         self.cameras.iter_mut().for_each( |o| o.update_input(&mut protocols) );
        
//         self.uis.iter_mut().for_each( |o| o.update_position(&mut protocols) );
//         self.cameras.iter_mut().for_each( |o| o.update_position(&mut protocols) );
        
//         let pl_upd_it = Player::updaters();
//         let ob_upd_it = Obstacle::updaters();
        
//         for updater in pl_upd_it.iter().zip(ob_upd_it.iter()) {
//             let (pl_upd, ob_upd) = updater;
//             for o in &mut self.players {
//                 pl_upd(o);
//             }
//             for o in &mut self.obstacles {
//                 ob_upd(o);
//             }
//         }
    }
}