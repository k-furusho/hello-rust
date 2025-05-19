use crate::domain::model::game::GameId;
use crate::domain::model::player::PlayerId;
use crate::domain::repository::game_repository::GameRepository;

pub struct ExchangeCardsUseCase<R: GameRepository> {
    game_repository: R,
}

pub struct ExchangeCardsParams {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_indices: Vec<usize>,
}

impl<R: GameRepository> ExchangeCardsUseCase<R> {
    pub fn new(game_repository: R) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&mut self, params: ExchangeCardsParams) -> Result<(), String> {
        // ゲームを取得
        let mut game = self.game_repository
            .find_by_id(&params.game_id)
            .ok_or_else(|| format!("ゲーム {} が見つかりません", params.game_id.value()))?;
        
        // プレイヤーのインデックスを取得
        let player_index = game.players().iter()
            .position(|p| p.id() == &params.player_id)
            .ok_or_else(|| format!("プレイヤー {} がこのゲームに参加していません", params.player_id))?;
        
        // カード交換を実行
        game.exchange_cards(player_index, &params.card_indices)
            .map_err(|e| e.to_string())?;
        
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
    
    // テスト用のゲーム作成
    fn create_test_game() -> Game {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        // プレイヤーを追加
        game.add_player(Player::new("プレイヤー1".to_string(), 1000)).unwrap();
        game.add_player(Player::new("プレイヤー2".to_string(), 1000)).unwrap();
        
        // ゲーム開始
        game.start_game().unwrap();
        game.deal_cards().unwrap();
        
        game
    }
    
    #[test]
    fn カード交換実行_正常系() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        let mut game = create_test_game();
        let game_id = game.id().clone();
        let player_id = game.players()[0].id().clone();
        
        // ドローフェーズに設定
        // 便宜上、privateフィールドに直接アクセスできない代わりに状態を再現する
        game = Game::from_serialized(
            game.id().clone(),
            game.variant(),
            game.players().to_vec(),
            game.community_cards().to_vec(),
            game.pot().total(),
            GamePhase::Drawing,
            game.current_round(),
            game.current_player_index(),
            game.dealer_index(),
            game.small_blind(),
            game.big_blind(),
            game.current_bet()
        ).unwrap();
        
        // 手札の最初のカードを記録
        let initial_cards = game.players()[0].hand().cards().to_vec();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = ExchangeCardsUseCase::new(game_repo.clone());
        
        let params = ExchangeCardsParams {
            game_id: game_id.clone(),
            player_id,
            card_indices: vec![0], // 最初のカードを交換
        };
        
        let result = usecase.execute(params);
        assert!(result.is_ok(), "カード交換の実行に失敗: {}", result.err().unwrap_or_default());
        
        // ゲームの状態を確認
        let updated_game = game_repo.find_by_id(&game_id).unwrap();
        let updated_cards = updated_game.players()[0].hand().cards();
        
        // 最初のカードが交換されていることを確認
        assert_ne!(updated_cards[0], initial_cards[0], "カードが交換されていません");
    }
    
    #[test]
    fn 存在しないゲームでのカード交換() {
        let game_repo = InMemoryGameRepository::new();
        let mut usecase = ExchangeCardsUseCase::new(game_repo);
        
        let params = ExchangeCardsParams {
            game_id: GameId::new(), // 存在しないゲームID
            player_id: PlayerId::new(),
            card_indices: vec![0],
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("見つかりません"));
    }
    
    #[test]
    fn 存在しないプレイヤーのカード交換() {
        let mut game_repo = InMemoryGameRepository::new();
        let game = create_test_game();
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        let mut usecase = ExchangeCardsUseCase::new(game_repo);
        
        let params = ExchangeCardsParams {
            game_id,
            player_id: PlayerId::new(), // 存在しないプレイヤーID
            card_indices: vec![0],
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("参加していません"));
    }
    
    #[test]
    fn 無効なカードインデックス() {
        let mut game_repo = InMemoryGameRepository::new();
        let mut game = create_test_game();
        let game_id = game.id().clone();
        let player_id = game.players()[0].id().clone();
        
        // ドローフェーズに設定
        game = Game::from_serialized(
            game.id().clone(),
            game.variant(),
            game.players().to_vec(),
            game.community_cards().to_vec(),
            game.pot().total(),
            GamePhase::Drawing,
            game.current_round(),
            game.current_player_index(),
            game.dealer_index(),
            game.small_blind(),
            game.big_blind(),
            game.current_bet()
        ).unwrap();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        let mut usecase = ExchangeCardsUseCase::new(game_repo);
        
        let params = ExchangeCardsParams {
            game_id,
            player_id,
            card_indices: vec![10], // 無効なインデックス
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("無効なカードインデックス"));
    }
} 