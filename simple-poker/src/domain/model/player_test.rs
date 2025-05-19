#[cfg(test)]
mod tests {
    use crate::domain::model::player::{Player, PlayerId};
    use crate::domain::model::card::{Card, Suit};

    #[test]
    fn プレイヤー作成() {
        let player = Player::new("テストプレイヤー".to_string(), 1000);
        assert_eq!(player.name(), "テストプレイヤー", "プレイヤー名が一致しません");
        assert_eq!(player.chips(), 1000, "チップ数が一致しません");
        assert_eq!(player.current_bet(), 0, "初期ベット額が0ではありません");
        assert!(!player.is_folded(), "初期状態でフォールドになっています");
        assert!(!player.is_all_in(), "初期状態でオールインになっています");
        assert!(!player.is_dealer(), "初期状態でディーラーになっています");
        assert!(player.hand().is_empty(), "初期手札が空ではありません");
    }

    #[test]
    fn プレイヤーid一意性() {
        let player1 = Player::new("プレイヤー1".to_string(), 1000);
        let player2 = Player::new("プレイヤー2".to_string(), 1000);
        assert_ne!(player1.id().value(), player2.id().value(), "プレイヤーIDが重複しています");
    }

    #[test]
    fn ベット処理_通常とオールイン() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        // 有効なベット
        let result = player.place_bet(500);
        assert!(result.is_ok(), "ベット処理に失敗しました");
        assert_eq!(result.unwrap(), 500, "ベット額が一致しません");
        assert_eq!(player.chips(), 500, "チップ数が減っていません");
        assert_eq!(player.current_bet(), 500, "現在のベット額が一致しません");
        assert!(!player.is_all_in(), "オールイン状態になっています");
        // 残りのチップすべてをベット（オールイン）
        let result = player.place_bet(500);
        assert!(result.is_ok(), "オールインベットに失敗しました");
        assert_eq!(result.unwrap(), 500, "オールインベット額が一致しません");
        assert_eq!(player.chips(), 0, "オールイン後のチップが0ではありません");
        assert_eq!(player.current_bet(), 1000, "オールイン後のベット額が一致しません");
        assert!(player.is_all_in(), "オールイン状態になっていません");
    }

    #[test]
    fn ベット処理_所持チップ以上() {
        let mut player = Player::new("テストプレイヤー".to_string(), 500);
        // 持っているチップ以上をベットしようとする（オールインになるはず）
        let result = player.place_bet(1000);
        assert!(result.is_ok(), "オールインベットに失敗しました");
        assert_eq!(result.unwrap(), 500, "実際にベットされた額が一致しません");
        assert_eq!(player.chips(), 0, "オールイン後のチップが0ではありません");
        assert_eq!(player.current_bet(), 500, "オールイン後のベット額が一致しません");
        assert!(player.is_all_in(), "オールイン状態になっていません");
    }

    #[test]
    fn フォールド処理() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        assert!(!player.is_folded(), "初期状態でフォールドになっています");
        player.fold();
        assert!(player.is_folded(), "フォールド処理が反映されていません");
    }

    #[test]
    fn ベットリセット() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        let _ = player.place_bet(500);
        assert_eq!(player.current_bet(), 500, "ベット額が一致しません");
        player.reset_bet();
        assert_eq!(player.current_bet(), 0, "ベットリセット後のベット額が0ではありません");
        assert_eq!(player.chips(), 500, "ベットリセット後のチップ数が一致しません");
    }

    #[test]
    fn 新ラウンド用リセット() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        let _ = player.place_bet(500);
        player.fold();
        let card = Card::new(Suit::Spade, 1).unwrap();
        let _ = player.hand_mut().add_card(card);
        assert_eq!(player.current_bet(), 500, "ベット額が一致しません");
        assert!(player.is_folded(), "フォールド状態になっていません");
        assert_eq!(player.hand().size(), 1, "手札サイズが一致しません");
        player.reset_for_new_round();
        assert_eq!(player.current_bet(), 0, "新ラウンドリセット後のベット額が0ではありません");
        assert!(!player.is_folded(), "新ラウンドリセット後もフォールド状態です");
        assert!(player.hand().is_empty(), "新ラウンドリセット後の手札が空ではありません");
        assert_eq!(player.chips(), 500, "新ラウンドリセット後のチップ数が一致しません");
    }

    #[test]
    fn 新ゲーム用リセット() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        let _ = player.place_bet(1000);
        player.fold();
        assert!(player.is_all_in(), "オールイン状態になっていません");
        player.set_dealer(true);
        assert!(player.is_dealer(), "ディーラー状態になっていません");
        player.reset_for_new_game();
        assert_eq!(player.current_bet(), 0, "新ゲームリセット後のベット額が0ではありません");
        assert!(!player.is_folded(), "新ゲームリセット後もフォールド状態です");
        assert!(!player.is_all_in(), "新ゲームリセット後もオールイン状態です");
        assert!(!player.is_dealer(), "新ゲームリセット後もディーラー状態です");
        assert!(player.hand().is_empty(), "新ゲームリセット後の手札が空ではありません");
    }

    #[test]
    fn シリアライズからの復元() {
        let player_id = PlayerId::new();
        let name = "テストプレイヤー".to_string();
        let chips = 1000;
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Heart, 10).unwrap();
        let cards = vec![card1, card2];
        let current_bet = 500;
        let is_folded = false;
        let is_all_in = false;
        let is_dealer = true;
        let result = Player::from_serialized(
            player_id.clone(),
            name.clone(),
            chips,
            cards.clone(),
            current_bet,
            is_folded,
            is_all_in,
            is_dealer
        );
        assert!(result.is_ok(), "シリアライズからの復元に失敗しました");
        let player = result.unwrap();
        assert_eq!(player.id().value(), player_id.value(), "IDが一致しません");
        assert_eq!(player.name(), &name, "名前が一致しません");
        assert_eq!(player.chips(), chips, "チップ数が一致しません");
        assert_eq!(player.current_bet(), current_bet, "ベット額が一致しません");
        assert_eq!(player.is_folded(), is_folded, "フォールド状態が一致しません");
        assert_eq!(player.is_all_in(), is_all_in, "オールイン状態が一致しません");
        assert_eq!(player.is_dealer(), is_dealer, "ディーラー状態が一致しません");
        assert_eq!(player.hand().size(), 2, "手札サイズが一致しません");
        assert_eq!(player.hand().cards()[0].suit(), Suit::Spade, "1枚目のスートが一致しません");
        assert_eq!(player.hand().cards()[0].rank(), 1, "1枚目のランクが一致しません");
        assert_eq!(player.hand().cards()[1].suit(), Suit::Heart, "2枚目のスートが一致しません");
        assert_eq!(player.hand().cards()[1].rank(), 10, "2枚目のランクが一致しません");
    }
} 