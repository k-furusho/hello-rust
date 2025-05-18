use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::model::game::GameId;
use crate::domain::model::player::PlayerId;
use crate::domain::repository::game_history_repository::{GameHistoryEntry, GameHistoryRepository};
use crate::domain::error::{RepositoryError, RepositoryResult};

#[derive(Clone)]
pub struct InMemoryGameHistoryRepository {
    entries: Arc<Mutex<HashMap<String, GameHistoryEntry>>>,
}

impl InMemoryGameHistoryRepository {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryGameHistoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl GameHistoryRepository for InMemoryGameHistoryRepository {
    fn save(&mut self, entry: &GameHistoryEntry) -> RepositoryResult<()> {
        let mut entries = self.entries.lock()
            .map_err(|_| RepositoryError::LockError("ロックの取得に失敗しました".to_string()))?;
        
        let key = format!("{}_{}", entry.game_id.value(), entry.timestamp.to_rfc3339());
        entries.insert(key, entry.clone());
        Ok(())
    }
    
    fn find_by_game_id(&self, game_id: &GameId) -> Option<GameHistoryEntry> {
        let entries = match self.entries.lock() {
            Ok(e) => e,
            Err(_) => return None,
        };
        
        entries.values()
            .filter(|e| e.game_id.value() == game_id.value())
            .max_by_key(|e| e.timestamp)
            .cloned()
    }
    
    fn find_by_player_id(&self, player_id: &PlayerId) -> Vec<GameHistoryEntry> {
        let entries = match self.entries.lock() {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };
        
        entries.values()
            .filter(|e| e.winner_ids.iter().any(|id| id.value() == player_id.value()))
            .cloned()
            .collect()
    }
    
    fn find_all(&self) -> Vec<GameHistoryEntry> {
        let entries = match self.entries.lock() {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };
        
        entries.values().cloned().collect()
    }
} 