use crate::domain::model::game::{Game, GameId};
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::error::{GameError, GameResult};

pub struct LoadGameUseCase<G: GameRepository> {
    game_repository: G,
}

impl<G: GameRepository> LoadGameUseCase<G> {
    pub fn new(game_repository: G) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&self, game_id: &GameId) -> GameResult<Game> {
        // ゲームを取得
        self.game_repository.find_by_id(game_id)
            .ok_or(GameError::GameNotFound(game_id.value().to_string()))
    }
    
    pub fn list_saved_games(&self) -> Vec<(GameId, String)> {
        // 保存されているゲーム一覧を取得
        self.game_repository.find_all()
            .into_iter()
            .map(|game| (game.id().clone(), game.variant().name().to_string()))
            .collect()
    }
} 