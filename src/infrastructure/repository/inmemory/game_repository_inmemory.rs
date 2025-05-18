use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::model::game::{Game, GameId};
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::error::{RepositoryError, RepositoryResult};

#[derive(Clone)]
pub struct InMemoryGameRepository {
    games: Arc<Mutex<HashMap<String, Game>>>,
}

impl InMemoryGameRepository {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryGameRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl GameRepository for InMemoryGameRepository {
    fn save(&mut self, game: &Game) -> RepositoryResult<()> {
        let mut games = self.games.lock()
            .map_err(|_| RepositoryError::LockError("ロックの取得に失敗しました".to_string()))?;
        games.insert(game.id().value().to_string(), game.clone());
        Ok(())
    }
    
    fn find_by_id(&self, id: &GameId) -> Option<Game> {
        let games = match self.games.lock() {
            Ok(games) => games,
            Err(_) => return None,
        };
        games.get(id.value()).cloned()
    }
    
    fn find_all(&self) -> Vec<Game> {
        let games = match self.games.lock() {
            Ok(games) => games,
            Err(_) => return Vec::new(),
        };
        games.values().cloned().collect()
    }
    
    fn delete(&mut self, id: &GameId) -> RepositoryResult<()> {
        let mut games = self.games.lock()
            .map_err(|_| RepositoryError::LockError("ロックの取得に失敗しました".to_string()))?;
        games.remove(id.value());
        Ok(())
    }
} 