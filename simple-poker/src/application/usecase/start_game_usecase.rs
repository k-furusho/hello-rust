use crate::domain::model::game::GameId;
use crate::domain::repository::game_repository::GameRepository;

pub struct StartGameUseCase<R: GameRepository> {
    game_repository: R,
}

impl<R: GameRepository> StartGameUseCase<R> {
    pub fn new(game_repository: R) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&mut self, game_id: &GameId) -> Result<(), String> {
        // ゲームを取得
        let mut game = self.game_repository
            .find_by_id(game_id)
            .ok_or_else(|| format!("ゲーム {} が見つかりません", game_id.value()))?;
        
        // ゲームを開始
        game.start_game().map_err(|e| e.to_string())?;
        
        // カードを配る
        game.deal_cards().map_err(|e| e.to_string())?;
        
        // ホールデムとオマハの場合はブラインドを投入
        if matches!(game.variant(), crate::domain::model::game::GameVariant::TexasHoldem | crate::domain::model::game::GameVariant::Omaha) {
            if let Some(crate::domain::model::game::BettingRound::PreFlop) = game.current_round() {
                game.post_blinds().map_err(|e| e.to_string())?;
            }
        }
        
        // 更新されたゲームを保存
        self.game_repository.save(&game)?;
        
        Ok(())
    }
} 