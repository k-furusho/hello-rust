use std::collections::HashMap;
use crate::domain::model::game::GameId;
use crate::domain::model::player::PlayerId;
use crate::domain::model::error::DomainError;
use crate::domain::repository::game_history_repository::{GameHistoryEntry, GameHistoryRepository};

#[derive(Default)]
pub struct InMemoryGameHistoryRepository {
    entries: HashMap<String, GameHistoryEntry>,
}

impl InMemoryGameHistoryRepository {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl GameHistoryRepository for InMemoryGameHistoryRepository {
    fn save(&mut self, entry: &GameHistoryEntry) -> Result<(), DomainError> {
        let key = format!("{}_{}", entry.game_id.value(), entry.timestamp.to_rfc3339());
        self.entries.insert(key, entry.clone());
        Ok(())
    }
    
    fn find_by_game_id(&self, game_id: &GameId) -> Option<GameHistoryEntry> {
        self.entries.values()
            .filter(|e| e.game_id.value() == game_id.value())
            .max_by_key(|e| e.timestamp)
            .cloned()
    }
    
    fn find_by_player_id(&self, player_id: &PlayerId) -> Vec<GameHistoryEntry> {
        self.entries.values()
            .filter(|e| e.winner_ids.iter().any(|id| id.value() == player_id.value()))
            .cloned()
            .collect()
    }
    
    fn find_all(&self) -> Vec<GameHistoryEntry> {
        self.entries.values().cloned().collect()
    }
} 