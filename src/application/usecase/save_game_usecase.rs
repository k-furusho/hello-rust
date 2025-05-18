use chrono::Utc;

use crate::domain::model::game::{Game, GameId};
use crate::domain::model::player::PlayerId;
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::repository::game_history_repository::{GameHistoryEntry, GameHistoryRepository};
use crate::domain::error::{GameError, GameResult, RepositoryError};

pub struct SaveGameUseCase<G: GameRepository, H: GameHistoryRepository> {
    game_repository: G,
    history_repository: H,
}

pub struct SaveGameParams {
    pub game_id: GameId,
    pub winner_ids: Vec<PlayerId>,
}

impl<G: GameRepository, H: GameHistoryRepository> SaveGameUseCase<G, H> {
    pub fn new(game_repository: G, history_repository: H) -> Self {
        Self {
            game_repository,
            history_repository,
        }
    }
    
    pub fn execute(&mut self, params: SaveGameParams) -> GameResult<()> {
        // ゲームを取得
        let game = self.game_repository.find_by_id(&params.game_id)
            .ok_or(GameError::GameNotFound(params.game_id.value().to_string()))?;
        
        // ゲームを保存
        self.game_repository.save(&game)
            .map_err(|e| GameError::Other(format!("ゲームの保存に失敗しました: {}", e)))?;
        
        // 履歴エントリを作成
        let history_entry = GameHistoryEntry {
            game_id: params.game_id,
            timestamp: Utc::now(),
            winner_ids: params.winner_ids,
            pot_amount: game.pot().total(),
            variant: game.variant().name().to_string(),
            player_count: game.players().len(),
        };
        
        // 履歴を保存
        self.history_repository.save(&history_entry)
            .map_err(|e| GameError::Other(format!("ゲーム履歴の保存に失敗しました: {}", e)))?;
        
        Ok(())
    }
} 