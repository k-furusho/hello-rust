use crate::domain::model::game::{Game, GameId, GameVariant};
use crate::domain::repository::game_repository::GameRepository;

pub struct CreateGameUseCase<R: GameRepository> {
    game_repository: R,
}

pub struct CreateGameParams {
    pub variant: GameVariant,
    pub small_blind: u32,
    pub big_blind: u32,
}

impl<R: GameRepository> CreateGameUseCase<R> {
    pub fn new(game_repository: R) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&mut self, params: CreateGameParams) -> Result<GameId, String> {
        let game = Game::new(params.variant, params.small_blind, params.big_blind)
            .map_err(|e| e.to_string())?;
        
        let game_id = game.id().clone();
        self.game_repository.save(&game)?;
        
        Ok(game_id)
    }
} 