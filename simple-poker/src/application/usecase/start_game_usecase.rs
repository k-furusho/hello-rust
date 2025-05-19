use crate::domain::model::game::GameId;
use crate::domain::repository::game_repository::GameRepository;

pub struct StartGameUseCase<R: GameRepository> {
    game_repository: R,
}

pub struct StartGameParams {
    pub game_id: GameId,
}

impl<R: GameRepository> StartGameUseCase<R> {
    pub fn new(game_repository: R) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&mut self, params: StartGameParams) -> Result<(), String> {
        // ゲームを取得
        let mut game = self.game_repository
            .find_by_id(&params.game_id)
            .ok_or_else(|| format!("ゲーム {} が見つかりません", params.game_id.value()))?;
        
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::game::{Game, GameVariant, GamePhase};
    use crate::domain::model::player::Player;
    use crate::infrastructure::repository::inmemory::game_repository_inmemory::InMemoryGameRepository;
    
    #[test]
    fn ゲーム開始_正常系() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        
        // ゲームを作成
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        // プレイヤーを追加（ゲーム開始には最低2人必要）
        game.add_player(Player::new("プレイヤー1".to_string(), 1000)).unwrap();
        game.add_player(Player::new("プレイヤー2".to_string(), 1000)).unwrap();
        
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = StartGameUseCase::new(game_repo.clone());
        
        let params = StartGameParams {
            game_id: game_id.clone(),
        };
        
        let result = usecase.execute(params);
        assert!(result.is_ok(), "ゲーム開始に失敗: {}", result.err().unwrap_or_default());
        
        // ゲームの状態を確認
        let updated_game = game_repo.find_by_id(&game_id).unwrap();
        
        // ゲームが開始されていることを確認
        assert_eq!(updated_game.current_phase(), GamePhase::Betting);
        
        // 各プレイヤーに手札が配られていることを確認
        for player in updated_game.players() {
            assert!(!player.hand().is_empty());
            assert_eq!(player.hand().cards().len(), 5); // 5枚のカードが配られている
        }
    }
    
    #[test]
    fn 存在しないゲームの開始() {
        let game_repo = InMemoryGameRepository::new();
        let mut usecase = StartGameUseCase::new(game_repo);
        
        let params = StartGameParams {
            game_id: GameId::new(), // 存在しないゲームID
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("見つかりません"));
    }
    
    #[test]
    fn プレイヤー不足でのゲーム開始失敗() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        
        // ゲームを作成
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        // プレイヤーを1人だけ追加（ゲーム開始には最低2人必要）
        game.add_player(Player::new("プレイヤー1".to_string(), 1000)).unwrap();
        
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = StartGameUseCase::new(game_repo);
        
        let params = StartGameParams {
            game_id,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("最低2人のプレイヤーが必要"));
    }
    
    #[test]
    fn 既に開始されたゲームの再開始() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        
        // ゲームを作成して開始
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        game.add_player(Player::new("プレイヤー1".to_string(), 1000)).unwrap();
        game.add_player(Player::new("プレイヤー2".to_string(), 1000)).unwrap();
        game.start_game().unwrap(); // 一度開始
        
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = StartGameUseCase::new(game_repo);
        
        let params = StartGameParams {
            game_id,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("既に開始"));
    }
} 