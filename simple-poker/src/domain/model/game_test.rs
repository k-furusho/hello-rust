#[cfg(test)]
mod tests {
    use crate::domain::model::game::{Game, GameVariant, GamePhase};
    use crate::domain::model::player::Player;

    #[test]
    fn ゲーム作成_有効なパラメータ() {
        // 有効なパラメータでゲームを作成
        let game = Game::new(GameVariant::FiveCardDraw, 5, 10);
        assert!(game.is_ok(), "ゲームの作成に失敗しました");
        
        let game = game.unwrap();
        assert_eq!(game.variant(), GameVariant::FiveCardDraw, "バリアントが一致しません");
        assert_eq!(game.small_blind(), 5, "スモールブラインドが一致しません");
        assert_eq!(game.big_blind(), 10, "ビッグブラインドが一致しません");
        assert_eq!(game.current_phase(), GamePhase::NotStarted, "初期フェーズが一致しません");
        assert!(game.current_round().is_none(), "初期ラウンドがNoneではありません");
        assert_eq!(game.players().len(), 0, "初期プレイヤー数が0ではありません");
    }
    
    #[test]
    fn ゲーム作成_無効なブラインド() {
        // スモールブラインドがビッグブラインドより大きい場合
        let game = Game::new(GameVariant::FiveCardDraw, 20, 10);
        assert!(game.is_err(), "不正なブラインドでもエラーになりません");
    }
    
    #[test]
    fn プレイヤー追加_正常系() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let player = Player::new("テストプレイヤー".to_string(), 1000);
        
        // プレイヤーを追加
        let result = game.add_player(player.clone());
        assert!(result.is_ok(), "プレイヤー追加に失敗しました");
        assert_eq!(game.players().len(), 1, "プレイヤー数が1ではありません");
        assert_eq!(game.players()[0].name(), "テストプレイヤー", "プレイヤー名が一致しません");
    }
    
    #[test]
    fn ゲーム開始後_プレイヤー追加不可() {
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
        assert!(result.is_err(), "ゲーム開始後にプレイヤーが追加できてしまいます");
    }
    
    #[test]
    fn ゲーム状態遷移() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let player1 = Player::new("プレイヤー1".to_string(), 1000);
        let player2 = Player::new("プレイヤー2".to_string(), 1000);
        
        // プレイヤーを追加
        let _ = game.add_player(player1);
        let _ = game.add_player(player2);
        
        // 開始前の状態を確認
        assert_eq!(game.current_phase(), GamePhase::NotStarted, "開始前フェーズが一致しません");
        
        // ゲーム開始
        let result = game.start_game();
        assert!(result.is_ok(), "ゲーム開始に失敗しました");
        assert_eq!(game.current_phase(), GamePhase::Dealing, "Dealingフェーズに遷移しません");
        
        // カードを配る
        let result = game.deal_cards();
        assert!(result.is_ok(), "カード配布に失敗しました");
        assert_eq!(game.current_phase(), GamePhase::Betting, "Bettingフェーズに遷移しません");
        
        // 無効な状態遷移を試みる（ベッティングからDealingへの遷移は不可）
        let initial_phase = game.current_phase();
        let result = game.deal_cards(); // 既にカードを配った後で再度配ろうとする
        assert!(result.is_err(), "不正な状態遷移でもエラーになりません");
        assert_eq!(game.current_phase(), initial_phase, "状態が変わってしまいました");
    }

    #[test]
    fn 最大プレイヤー数制限() {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        // 最大プレイヤー数（10人）までプレイヤーを追加
        for i in 0..10 {
            let player = Player::new(format!("プレイヤー{}", i), 1000);
            let result = game.add_player(player);
            assert!(result.is_ok(), "{}人目のプレイヤー追加に失敗", i+1);
        }
        
        assert_eq!(game.players().len(), 10, "プレイヤー数が10ではありません");
        
        // 11人目のプレイヤーを追加しようとする
        let extra_player = Player::new("余分なプレイヤー".to_string(), 1000);
        let result = game.add_player(extra_player);
        
        // 最大数を超えるのでエラーになるはず
        assert!(result.is_err(), "11人目のプレイヤーが追加できてしまいます");
        assert_eq!(game.players().len(), 10, "プレイヤー数が10を超えています");
    }
    
    #[test]
    fn ゲームid一意性() {
        // 複数のゲームを作成してIDが異なることを確認
        let game1 = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let game2 = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        assert_ne!(game1.id().value(), game2.id().value(), "ゲームIDが重複しています");
    }
    
    #[test]
    fn 新しいハンド用リセット() {
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
        assert_eq!(game.pot().total(), 100, "ポット加算が反映されていません");
        
        // 新しいハンドのためにリセット
        let result = game.reset_for_new_hand();
        assert!(result.is_ok(), "リセットに失敗しました");
        
        // リセット後の状態を確認
        assert_eq!(game.current_phase(), GamePhase::NotStarted, "リセット後フェーズが一致しません");
        assert_eq!(game.pot().total(), 0, "リセット後ポットが0ではありません");
        assert_eq!(game.current_bet(), 0, "リセット後ベット額が0ではありません");
    }
} 