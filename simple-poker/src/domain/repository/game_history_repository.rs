use crate::domain::model::game::GameId;
use crate::domain::model::player::PlayerId;
use crate::domain::model::error::DomainError;

#[derive(Debug, Clone)]
pub struct GameHistoryEntry {
    pub game_id: GameId,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub winner_ids: Vec<PlayerId>,
    pub pot_amount: u32,
    pub variant: String,
    pub player_count: usize,
}

pub trait GameHistoryRepository {
    fn save(&mut self, entry: &GameHistoryEntry) -> Result<(), DomainError>;
    fn find_by_game_id(&self, game_id: &GameId) -> Option<GameHistoryEntry>;
    fn find_by_player_id(&self, player_id: &PlayerId) -> Vec<GameHistoryEntry>;
    fn find_all(&self) -> Vec<GameHistoryEntry>;
} 