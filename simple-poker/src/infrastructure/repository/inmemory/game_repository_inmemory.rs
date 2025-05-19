use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::model::game::{Game, GameId};
use crate::domain::model::error::DomainError;
use crate::domain::repository::game_repository::GameRepository;

/// ゲームIDとゲームのマッピングを表す型
type GameMap = HashMap<String, Game>;
/// スレッドセーフなゲームマップ
type ThreadSafeGameMap = Arc<Mutex<GameMap>>;

#[derive(Clone)]
pub struct InMemoryGameRepository {
    games: ThreadSafeGameMap,
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
    fn save(&mut self, game: &Game) -> Result<(), DomainError> {
        let mut games = self.games.lock().map_err(|_| DomainError::InvalidState("ロックの取得に失敗しました".into()))?;
        games.insert(game.id().value().to_string(), game.clone());
        Ok(())
    }
    
    fn find_by_id(&self, id: &GameId) -> Option<Game> {
        let games = self.games.lock().ok()?;
        games.get(id.value()).cloned()
    }
    
    fn find_all(&self) -> Vec<Game> {
        let games = match self.games.lock() {
            Ok(games) => games,
            Err(_) => return Vec::new(),
        };
        games.values().cloned().collect()
    }
    
    fn delete(&mut self, id: &GameId) -> Result<(), DomainError> {
        let mut games = self.games.lock().map_err(|_| DomainError::InvalidState("ロックの取得に失敗しました".into()))?;
        games.remove(id.value());
        Ok(())
    }
} 