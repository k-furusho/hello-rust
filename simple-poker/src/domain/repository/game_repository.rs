use crate::domain::model::game::{Game, GameId};
use crate::domain::model::error::DomainError;

pub trait GameRepository {
    fn save(&mut self, game: &Game) -> Result<(), DomainError>;
    fn find_by_id(&self, id: &GameId) -> Option<Game>;
    fn find_all(&self) -> Vec<Game>;
    fn delete(&mut self, id: &GameId) -> Result<(), DomainError>;
} 