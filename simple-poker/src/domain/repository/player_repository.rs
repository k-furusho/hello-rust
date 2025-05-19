use crate::domain::model::player::{Player, PlayerId};
use crate::domain::model::error::DomainError;

pub trait PlayerRepository {
    fn save(&mut self, player: &Player) -> Result<(), DomainError>;
    fn find_by_id(&self, id: &PlayerId) -> Option<Player>;
    fn find_all(&self) -> Vec<Player>;
    fn delete(&mut self, id: &PlayerId) -> Result<(), DomainError>;
} 