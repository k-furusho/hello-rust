use crate::domain::model::bet::BetAction;
use crate::domain::model::game::{GameId, GameSerializedData};
use crate::domain::model::player::PlayerId;
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::service::game_rule::GameRuleService;

pub struct PlaceBetUseCase<R: GameRepository> {
    game_repository: R,
}

pub struct PlaceBetParams {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub action: BetAction,
    pub bet_amount: Option<u32>,
}

impl<R: GameRepository> PlaceBetUseCase<R> {
    pub fn new(game_repository: R) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&mut self, params: PlaceBetParams) -> Result<(), String> {
        // ゲームを取得
        let mut game = self.game_repository
            .find_by_id(&params.game_id)
            .ok_or_else(|| format!("ゲーム {} が見つかりません", params.game_id.value()))?;
        
        // プレイヤーのインデックスを取得
        let player_index = game.players().iter()
            .position(|p| p.id() == &params.player_id)
            .ok_or_else(|| format!("プレイヤー {} がこのゲームに参加していません", params.player_id))?;
        
        // プレイヤーのアクションを処理
        GameRuleService::process_action(
            &mut game,
            player_index,
            params.action,
            params.bet_amount
        ).map_err(|e| e.to_string())?;
        
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
    fn プレイヤーのアクション実行_チェック() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        let mut game = create_test_game();
        let game_id = game.id().clone();
        let player_id = game.players()[0].id().clone();
        
        // ベット額をゼロに設定
        game.set_current_bet(0);
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = PlaceBetUseCase::new(game_repo.clone());
        
        let params = PlaceBetParams {
            game_id: game_id.clone(),
            player_id: player_id.clone(),
            action: BetAction::Check,
            bet_amount: None,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_ok(), "チェックアクションの実行に失敗: {}", result.err().unwrap_or_default());
        
        // ゲームの状態を確認
        let updated_game = game_repo.find_by_id(&game_id).unwrap();
        assert_eq!(updated_game.current_bet(), 0);
        assert!(!updated_game.players()[0].is_folded());
    }
    
    #[test]
    fn プレイヤーのアクション実行_フォールド() {
        let mut game_repo = InMemoryGameRepository::new();
        let mut game = create_test_game();
        let game_id = game.id().clone();
        let player_id = game.players()[0].id().clone();
        
        // ベット額を設定
        game.set_current_bet(20);
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = PlaceBetUseCase::new(game_repo.clone());
        
        let params = PlaceBetParams {
            game_id: game_id.clone(),
            player_id,
            action: BetAction::Fold,
            bet_amount: None,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_ok());
        
        // プレイヤーがフォールドしたことを確認
        let updated_game = game_repo.find_by_id(&game_id).unwrap();
        assert!(updated_game.players()[0].is_folded());
    }
    
    #[test]
    fn プレイヤーのアクション実行_レイズ() {
        let mut game_repo = InMemoryGameRepository::new();
        let mut game = create_test_game();
        let game_id = game.id().clone();
        let player_id = game.players()[0].id().clone();
        
        // 初期チップ量を記録
        let initial_chips = game.players()[0].chips();
        
        // ベッティングフェーズに設定
        game = Game::from_serialized(
            GameSerializedData {
                id: game.id().clone(),
                variant: game.variant(),
                players: game.players().to_vec(),
                community_cards: game.community_cards().to_vec(),
                pot_total: 0, // ポット額を0に初期化
                current_phase: GamePhase::Betting,
                current_round: game.current_round(),
                current_player_index: 0, // カレントプレイヤーを0に設定
                dealer_index: game.dealer_index(),
                small_blind: game.small_blind(),
                big_blind: game.big_blind(),
                current_bet: 10 // 現在のベット額を10に設定
            }
        ).unwrap();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = PlaceBetUseCase::new(game_repo.clone());
        
        let params = PlaceBetParams {
            game_id: game_id.clone(),
            player_id,
            action: BetAction::Raise,
            bet_amount: Some(30), // 30にレイズ
        };
        
        let result = usecase.execute(params);
        
        // エラーがあれば詳細を表示
        if let Err(ref err) = result {
            println!("レイズエラー: {}", err);
        }
        
        assert!(result.is_ok(), "レイズに失敗: {}", result.err().unwrap_or_default());
        
        // ゲームの状態を確認
        let updated_game = game_repo.find_by_id(&game_id).unwrap();
        
        // デバッグ情報を出力
        println!("現在のフェーズ: {:?}", updated_game.current_phase());
        println!("カレントベット: {}", updated_game.current_bet());
        println!("プレイヤーのベット: {}", updated_game.players()[0].current_bet());
        println!("プレイヤーの初期チップ: {}", initial_chips);
        println!("プレイヤーの現在チップ: {}", updated_game.players()[0].chips());
        println!("ポット額: {}", updated_game.pot().total());
        
        // カレントベットが更新されていることを確認
        assert_eq!(updated_game.current_bet(), 30, "カレントベットが30に更新されていません: {}", updated_game.current_bet());
        
        // チップが減少していることを確認
        assert_eq!(updated_game.players()[0].chips(), initial_chips - 30, "プレイヤーのチップが正しく減少していません");
        
        // ポット額が増加していることを確認
        assert_eq!(updated_game.pot().total(), 30, "ポット額が正しく増加していません");
    }
    
    #[test]
    fn 存在しないゲームへのアクション() {
        let game_repo = InMemoryGameRepository::new();
        let mut usecase = PlaceBetUseCase::new(game_repo);
        
        let params = PlaceBetParams {
            game_id: GameId::new(),
            player_id: PlayerId::new(),
            action: BetAction::Check,
            bet_amount: None,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("見つかりません"));
    }
    
    #[test]
    fn 存在しないプレイヤーのアクション() {
        let mut game_repo = InMemoryGameRepository::new();
        let game = create_test_game();
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        let mut usecase = PlaceBetUseCase::new(game_repo);
        
        let params = PlaceBetParams {
            game_id,
            player_id: PlayerId::new(), // 存在しないプレイヤーID
            action: BetAction::Check,
            bet_amount: None,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("参加していません"));
    }
} 