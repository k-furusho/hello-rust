#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::card::{Card, Suit};
    use crate::domain::model::player::Player;
    use crate::domain::model::game::{Game, GameVariant};

    // テスト用のゲーム作成
    fn create_test_game() -> Game {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        
        // プレイヤーを追加
        game.add_player(Player::new("プレイヤー1".to_string(), 1000)).unwrap();
        game.add_player(Player::new("プレイヤー2".to_string(), 1000)).unwrap();
        game.add_player(Player::new("プレイヤー3".to_string(), 1000)).unwrap();
        
        // ゲーム開始
        game.start_game().unwrap();
        game.deal_cards().unwrap();
        
        game
    }

    #[test]
    fn 使用可能なアクション取得() {
        let mut game = create_test_game();
        
        // ベッティングフェーズに移行
        game.set_current_bet(0);
        
        // プレイヤー0のアクション
        let actions = GameRuleService::available_actions(&game, 0);
        
        // ベットがない場合、チェック可能
        assert!(actions.contains(&BetAction::Fold));
        assert!(actions.contains(&BetAction::Check));
        assert!(actions.contains(&BetAction::Raise));
        assert!(actions.contains(&BetAction::AllIn));
        
        // ベットがある場合
        game.set_current_bet(50);
        let actions = GameRuleService::available_actions(&game, 1);
        
        // チェックはできず、コールが可能
        assert!(actions.contains(&BetAction::Fold));
        assert!(!actions.contains(&BetAction::Check));
        assert!(actions.contains(&BetAction::Call));
        assert!(actions.contains(&BetAction::Raise));
        assert!(actions.contains(&BetAction::AllIn));
    }

    #[test]
    fn アクション処理_フォールド() {
        let mut game = create_test_game();
        
        // プレイヤー0がフォールド
        GameRuleService::process_action(&mut game, 0, BetAction::Fold, None).unwrap();
        
        // フォールド状態確認
        assert!(game.players()[0].is_folded());
        assert_eq!(game.pot().total(), 0); // ポットは変わらない
    }

    #[test]
    fn アクション処理_チェック() {
        let mut game = create_test_game();
        
        // ベットがない状態
        game.set_current_bet(0);
        
        // プレイヤー0がチェック
        GameRuleService::process_action(&mut game, 0, BetAction::Check, None).unwrap();
        
        // チェック後の状態確認
        assert!(!game.players()[0].is_folded());
        assert_eq!(game.pot().total(), 0); // ポットは変わらない
        assert_eq!(game.current_bet(), 0); // ベット額も変わらない
    }

    #[test]
    fn アクション処理_コール() {
        let mut game = create_test_game();
        
        // 現在のベット額を設定
        game.set_current_bet(50);
        
        // プレイヤー0がコール
        GameRuleService::process_action(&mut game, 0, BetAction::Call, None).unwrap();
        
        // コール後の状態確認
        assert!(!game.players()[0].is_folded());
        assert_eq!(game.pot().total(), 50); // ポットにコール額が追加
        assert_eq!(game.players()[0].current_bet(), 50); // プレイヤーのベット額が更新
    }

    #[test]
    fn アクション処理_レイズ() {
        let mut game = create_test_game();
        
        // 現在のベット額を設定
        game.set_current_bet(20);
        
        // プレイヤー0がレイズ（現在のベット20 + ビッグブラインド10 = 30以上である必要がある）
        GameRuleService::process_action(&mut game, 0, BetAction::Raise, Some(100)).unwrap();
        
        // レイズ後の状態確認
        assert!(!game.players()[0].is_folded());
        assert_eq!(game.pot().total(), 100); // ポットにレイズ額が追加
        assert_eq!(game.current_bet(), 100); // 現在のベット額が更新
        assert_eq!(game.players()[0].current_bet(), 100); // プレイヤーのベット額が更新
    }

    #[test]
    fn アクション処理_レイズ_最低額以下でエラー() {
        let mut game = create_test_game();
        
        // 現在のベット額を設定
        game.set_current_bet(20);
        
        // 最低レイズ額は現在のベット20 + ビッグブラインド10 = 30
        // 25でレイズしようとするとエラー
        let result = GameRuleService::process_action(&mut game, 0, BetAction::Raise, Some(25));
        assert!(result.is_err());
    }

    #[test]
    fn アクション処理_オールイン() {
        let mut game = create_test_game();
        
        // プレイヤー0のチップを減らす（テスト用）
        game.players_mut()[0].place_bet(900).unwrap(); // 残り100チップ
        
        // プレイヤー0がオールイン
        GameRuleService::process_action(&mut game, 0, BetAction::AllIn, None).unwrap();
        
        // オールイン後の状態確認
        assert!(game.players()[0].is_all_in());
        assert_eq!(game.pot().total(), 100 + 900); // ポットに残りのチップが追加
        assert_eq!(game.current_bet(), 100 + 900); // 現在のベット額が更新
        assert_eq!(game.players()[0].chips(), 0); // プレイヤーのチップが0になる
    }

    #[test]
    fn ベッティングラウンド完了判定_全員アクション終了() {
        let mut game = create_test_game();
        
        // ベット設定
        game.set_current_bet(50);
        
        // 全プレイヤーが同額をベット
        for i in 0..3 {
            game.players_mut()[i].place_bet(50).unwrap();
        }
        
        // これでラウンド終了と判定される
        assert!(GameRuleService::is_betting_round_complete(&game));
    }

    #[test]
    fn ベッティングラウンド完了判定_全員アクション終了していない() {
        let mut game = create_test_game();
        
        // ベット設定
        game.set_current_bet(50);
        
        // プレイヤー0と1はベット済み
        game.players_mut()[0].place_bet(50).unwrap();
        game.players_mut()[1].place_bet(50).unwrap();
        // プレイヤー2はまだベットしていない
        
        // ラウンド終了と判定されない
        assert!(!GameRuleService::is_betting_round_complete(&game));
    }

    #[test]
    fn ベッティングラウンド完了判定_フォールドが多い() {
        let mut game = create_test_game();
        
        // プレイヤー1と2がフォールド
        game.players_mut()[1].fold();
        game.players_mut()[2].fold();
        
        // 残りのアクティブプレイヤーが1人だけの場合、ラウンド終了
        assert!(GameRuleService::is_betting_round_complete(&game));
    }

    #[test]
    fn 勝者決定_単一() {
        let mut game = create_test_game();
        
        // プレイヤー1と2がフォールド
        game.players_mut()[1].fold();
        game.players_mut()[2].fold();
        
        // 勝者の決定
        let winners = GameRuleService::determine_winners(&game);
        
        // プレイヤー0だけが勝者
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].0, 0);
    }

    #[test]
    fn ポット分配() {
        let mut game = create_test_game();
        
        // ポットに追加
        game.pot_mut().add(300);
        
        // プレイヤー0を勝者にする
        game.players_mut()[1].fold();
        game.players_mut()[2].fold();
        
        // ポット分配
        let result = GameRuleService::distribute_pot(&mut game);
        assert!(result.is_ok());
        
        // 勝者にポット額が分配される
        assert_eq!(game.players()[0].chips(), 1300); // 元の1000 + ポット300
        assert_eq!(game.pot().total(), 0); // ポットは空になる
    }

    #[test]
    fn ポット分配_複数勝者() {
        let mut game = create_test_game();
        
        // ショーダウンフェーズに設定（テスト用）
        game.set_current_phase(GamePhase::Showdown);
        
        // ポットに追加
        game.pot_mut().add(300);
        
        // プレイヤー0と1に同じ手札を持たせる（テスト用にランクを同じにする）
        let cards = vec![
            Card::new(Suit::Heart, 10).unwrap(),
            Card::new(Suit::Diamond, 10).unwrap(),
            Card::new(Suit::Club, 10).unwrap(),
            Card::new(Suit::Spade, 10).unwrap(),
            Card::new(Suit::Heart, 13).unwrap(),
        ];
        
        for card in &cards {
            game.players_mut()[0].hand_mut().add_card(*card).unwrap();
            game.players_mut()[1].hand_mut().add_card(*card).unwrap();
        }
        
        // 勝者決定（実装依存のため、テスト可能かは状況による）
        let winners = vec![(0, "プレイヤー1".to_string()), (1, "プレイヤー2".to_string())];
        
        // ポット分配
        let distribution = GameRuleService::distribute_pot(&mut game).unwrap();
        
        // ポットは等分される
        assert_eq!(distribution.len(), 2);
        assert_eq!(distribution[0].1, 150); // 300/2 = 150
        assert_eq!(distribution[1].1, 150);
        
        // プレイヤーのチップが増える
        assert_eq!(game.players()[0].chips(), 1150); // 元の1000 + 150
        assert_eq!(game.players()[1].chips(), 1150);
        
        // ポットは空になる
        assert_eq!(game.pot().total(), 0);
    }
} 