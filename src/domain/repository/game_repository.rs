use crate::domain::model::game::{Game, GameId};
use crate::domain::error::RepositoryResult;

pub trait GameRepository {
    fn save(&mut self, game: &Game) -> RepositoryResult<()>;
    fn find_by_id(&self, id: &GameId) -> Option<Game>;
    fn find_all(&self) -> Vec<Game>;
    fn delete(&mut self, id: &GameId) -> RepositoryResult<()>;
} 