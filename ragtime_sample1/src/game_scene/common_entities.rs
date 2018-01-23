use std::collections::HashMap;
use components::*;
use game_scene::entities::*;
use protocol::Protocol;
use game_scene::room::PlayerID;

#[derive(Default)]
pub struct CommonEntities {
    players: Vec<Player>,
    obstacles : Vec<Obstacle>,
}

impl CommonEntities {
    pub fn new() -> CommonEntities {
        CommonEntities {..Default::default()}
    }
    pub fn update(&mut self, commands: &Vec<(UserID,Command)>) {
        
        let mut protocols = Vec::new();
        self.players.iter_mut().for_each( |o| o.update_input(&mut protocols) );
        self.obstacles.iter_mut().for_each( |o| o.update_input(&mut protocols) );
        
        self.players.iter_mut().for_each( |o| o.update_position(&mut protocols) );
        self.obstacles.iter_mut().for_each( |o| o.update_position(&mut protocols) );
        
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