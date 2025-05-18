#[cfg(test)]
mod tests {
    use crate::domain::model::bet::BetAction;
    use crate::domain::model::game::{Game, GameVariant};
    use crate::domain::model::player::Player;
    use crate::domain::service::game_rule::GameRuleService;

    #[test]
    fn test_available_actions() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).expect("ゲーム作成失敗");
        
        let player1 = Player::new("プレイヤー1".to_string(), 100);
        let player2 = Player::new("プレイヤー2".to_string(), 100);
        
        game.add_player(player1).expect("プレイヤー追加失敗");
        game.add_player(player2).expect("プレイヤー追加失敗");
        
        game.start_game().expect("ゲーム開始失敗");
        game.deal_cards().expect("カード配布失敗");
        
        // プレイヤー1の可能なアクション
        let actions = GameRuleService::available_actions(&game, 0);
        
        // フォールド、チェック、レイズ、オールインが可能
        assert!(actions.contains(&BetAction::Fold));
        assert!(actions.contains(&BetAction::Check));
        assert!(actions.contains(&BetAction::Raise));
        assert!(actions.contains(&BetAction::AllIn));
        
        // ベットがない状態ではコールはできない
        assert!(!actions.contains(&BetAction::Call));
    }
    
    #[test]
    fn test_process_action_fold() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).expect("ゲーム作成失敗");
        
        let player1 = Player::new("プレイヤー1".to_string(), 100);
        let player2 = Player::new("プレイヤー2".to_string(), 100);
        
        game.add_player(player1).expect("プレイヤー追加失敗");
        game.add_player(player2).expect("プレイヤー追加失敗");
        
        game.start_game().expect("ゲーム開始失敗");
        game.deal_cards().expect("カード配布失敗");
        
        // プレイヤー1がフォールド
        GameRuleService::process_action(&mut game, 0, BetAction::Fold, None).expect("アクション実行失敗");
        
        // プレイヤー1がフォールドしていることを確認
        assert!(game.players()[0].is_folded());
    }
    
    #[test]
    fn test_process_action_raise() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).expect("ゲーム作成失敗");
        
        let player1 = Player::new("プレイヤー1".to_string(), 100);
        let player2 = Player::new("プレイヤー2".to_string(), 100);
        
        game.add_player(player1).expect("プレイヤー追加失敗");
        game.add_player(player2).expect("プレイヤー追加失敗");
        
        game.start_game().expect("ゲーム開始失敗");
        game.deal_cards().expect("カード配布失敗");
        
        // プレイヤー1がレイズ
        GameRuleService::process_action(&mut game, 0, BetAction::Raise, Some(20)).expect("アクション実行失敗");
        
        // ベット額とポットが期待通りか確認
        assert_eq!(game.current_bet(), 20);
        assert_eq!(game.pot().total(), 20);
        assert_eq!(game.players()[0].chips(), 80); // 100 - 20
    }
    
    #[test]
    fn test_determine_winners() {
        // 勝者判定のテストはHandEvaluationServiceに依存するため、
        // モックまたは簡略化したテストケースが必要です。
        // 実際の実装ではここでより詳細なテストを行います。
    }
} 