use crate::domain::model::player::{Player, PlayerId};

pub trait PlayerRepository {
    fn save(&mut self, player: &Player) -> Result<(), String>;
    fn find_by_id(&self, id: &PlayerId) -> Option<Player>;
    fn find_all(&self) -> Vec<Player>;
    fn delete(&mut self, id: &PlayerId) -> Result<(), String>;
} 