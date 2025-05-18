#[cfg(test)]
mod tests {
    use crate::domain::model::game::{Game, GameVariant, GamePhase};
    use crate::domain::model::player::Player;

    #[test]
    fn test_game_creation_with_valid_parameters() {
        // 有効なパラメータでゲームを作成
        let game = Game::new(GameVariant::FiveCardDraw, 5, 10);
        assert!(game.is_ok());
        
        let game = game.unwrap();
        assert_eq!(game.variant(), GameVariant::FiveCardDraw);
        assert_eq!(game.small_blind(), 5);
        assert_eq!(game.big_blind(), 10);
        assert_eq!(game.current_phase(), GamePhase::NotStarted);
        assert!(game.current_round().is_none());
        assert_eq!(game.players().len(), 0);
    }
    
    #[test]
    fn test_game_creation_with_invalid_blinds() {
        // スモールブラインドがビッグブラインドより大きい場合
        let game = Game::new(GameVariant::FiveCardDraw, 20, 10);
        assert!(game.is_err());
    }
    
    #[test]
    fn test_add_player_to_game() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let player = Player::new("テストプレイヤー".to_string(), 1000);
        
        // プレイヤーを追加
        let result = game.add_player(player.clone());
        assert!(result.is_ok());
        assert_eq!(game.players().len(), 1);
        assert_eq!(game.players()[0].name(), "テストプレイヤー");
    }
    
    #[test]
    fn test_add_player_when_game_started() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let player1 = Player::new("プレイヤー1".to_string(), 1000);
        let player2 = Player::new("プレイヤー2".to_string(), 1000);
        
        // 最初のプレイヤーを追加
        let _ = game.add_player(player1);
        let _ = game.add_player(player2);
        
        // ゲームを開始
        let _ = game.start_game();
        
        // ゲーム開始後にプレイヤーを追加しようとする
        let player3 = Player::new("プレイヤー3".to_string(), 1000);
        let result = game.add_player(player3);
        
        // ゲーム開始後はプレイヤーを追加できないはず
        assert!(result.is_err());
    }
    
    #[test]
    fn test_game_state_transitions() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let player1 = Player::new("プレイヤー1".to_string(), 1000);
        let player2 = Player::new("プレイヤー2".to_string(), 1000);
        
        // プレイヤーを追加
        let _ = game.add_player(player1);
        let _ = game.add_player(player2);
        
        // 開始前の状態を確認
        assert_eq!(game.current_phase(), GamePhase::NotStarted);
        
        // ゲーム開始
        let result = game.start_game();
        assert!(result.is_ok());
        assert_eq!(game.current_phase(), GamePhase::Dealing);
        
        // カードを配る
        let result = game.deal_cards();
        assert!(result.is_ok());
        assert_eq!(game.current_phase(), GamePhase::Betting);
        
        // 無効な状態遷移を試みる（ベッティングからDealingへの遷移は不可）
        let initial_phase = game.current_phase();
        let result = game.deal_cards(); // 既にカードを配った後で再度配ろうとする
        assert!(result.is_err());
        assert_eq!(game.current_phase(), initial_phase); // 状態は変わらないはず
    }

    #[test]
    fn test_max_players_limit() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        // 最大プレイヤー数（10人）までプレイヤーを追加
        for i in 0..10 {
            let player = Player::new(format!("プレイヤー{}", i), 1000);
            let result = game.add_player(player);
            assert!(result.is_ok());
        }
        
        assert_eq!(game.players().len(), 10);
        
        // 11人目のプレイヤーを追加しようとする
        let extra_player = Player::new("余分なプレイヤー".to_string(), 1000);
        let result = game.add_player(extra_player);
        
        // 最大数を超えるのでエラーになるはず
        assert!(result.is_err());
        assert_eq!(game.players().len(), 10); // プレイヤー数は変わらない
    }
    
    #[test]
    fn test_game_id_uniqueness() {
        // 複数のゲームを作成してIDが異なることを確認
        let game1 = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let game2 = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        assert_ne!(game1.id().value(), game2.id().value());
    }
    
    #[test]
    fn test_reset_for_new_hand() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let player1 = Player::new("プレイヤー1".to_string(), 1000);
        let player2 = Player::new("プレイヤー2".to_string(), 1000);
        
        let _ = game.add_player(player1);
        let _ = game.add_player(player2);
        
        // ゲームを開始してカードを配る
        let _ = game.start_game();
        let _ = game.deal_cards();
        
        // 状態を変更
        game.pot_mut().add(100);
        assert_eq!(game.pot().total(), 100);
        
        // 新しいハンドのためにリセット
        let result = game.reset_for_new_hand();
        assert!(result.is_ok());
        
        // リセット後の状態を確認
        assert_eq!(game.current_phase(), GamePhase::NotStarted);
        assert_eq!(game.pot().total(), 0);
        assert_eq!(game.current_bet(), 0);
    }
} 