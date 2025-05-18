#[cfg(test)]
mod tests {
    use crate::domain::model::player::{Player, PlayerId};
    use crate::domain::model::card::{Card, Suit};

    #[test]
    fn test_player_creation() {
        let player = Player::new("テストプレイヤー".to_string(), 1000);
        
        assert_eq!(player.name(), "テストプレイヤー");
        assert_eq!(player.chips(), 1000);
        assert_eq!(player.current_bet(), 0);
        assert!(!player.is_folded());
        assert!(!player.is_all_in());
        assert!(!player.is_dealer());
        assert!(player.hand().is_empty());
    }
    
    #[test]
    fn test_player_id_uniqueness() {
        let player1 = Player::new("プレイヤー1".to_string(), 1000);
        let player2 = Player::new("プレイヤー2".to_string(), 1000);
        
        assert_ne!(player1.id().value(), player2.id().value());
    }
    
    #[test]
    fn test_player_place_bet() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        
        // 有効なベット
        let result = player.place_bet(500);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 500);
        assert_eq!(player.chips(), 500);
        assert_eq!(player.current_bet(), 500);
        assert!(!player.is_all_in());
        
        // 残りのチップすべてをベット（オールイン）
        let result = player.place_bet(500);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 500);
        assert_eq!(player.chips(), 0);
        assert_eq!(player.current_bet(), 1000);
        assert!(player.is_all_in());
    }
    
    #[test]
    fn test_player_place_bet_more_than_chips() {
        let mut player = Player::new("テストプレイヤー".to_string(), 500);
        
        // 持っているチップ以上をベットしようとする（オールインになるはず）
        let result = player.place_bet(1000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 500); // 実際にベットされた額は500
        assert_eq!(player.chips(), 0);
        assert_eq!(player.current_bet(), 500);
        assert!(player.is_all_in());
    }
    
    #[test]
    fn test_player_fold() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        
        assert!(!player.is_folded());
        
        player.fold();
        assert!(player.is_folded());
    }
    
    #[test]
    fn test_player_reset_bet() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        
        // ベットする
        let _ = player.place_bet(500);
        assert_eq!(player.current_bet(), 500);
        
        // ベットをリセット
        player.reset_bet();
        assert_eq!(player.current_bet(), 0);
        assert_eq!(player.chips(), 500); // チップ自体は変わらない
    }
    
    #[test]
    fn test_player_reset_for_new_round() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        
        // ベットしてフォールドする
        let _ = player.place_bet(500);
        player.fold();
        
        // 手札を追加
        let card = Card::new(Suit::Spade, 1).unwrap();
        let _ = player.hand_mut().add_card(card);
        
        assert_eq!(player.current_bet(), 500);
        assert!(player.is_folded());
        assert_eq!(player.hand().size(), 1);
        
        // 新しいラウンドのためにリセット
        player.reset_for_new_round();
        
        assert_eq!(player.current_bet(), 0);
        assert!(!player.is_folded());
        assert!(player.hand().is_empty());
        assert_eq!(player.chips(), 500); // チップ自体は変わらない
    }
    
    #[test]
    fn test_player_reset_for_new_game() {
        let mut player = Player::new("テストプレイヤー".to_string(), 1000);
        
        // ベットしてフォールドしてオールインする
        let _ = player.place_bet(1000);
        player.fold();
        assert!(player.is_all_in());
        
        // ディーラーに設定
        player.set_dealer(true);
        assert!(player.is_dealer());
        
        // 新しいゲームのためにリセット
        player.reset_for_new_game();
        
        assert_eq!(player.current_bet(), 0);
        assert!(!player.is_folded());
        assert!(!player.is_all_in());
        assert!(!player.is_dealer());
        assert!(player.hand().is_empty());
    }
    
    #[test]
    fn test_player_from_serialized() {
        // シリアライズ/デシリアライズのテスト
        let player_id = PlayerId::new();
        let name = "テストプレイヤー".to_string();
        let chips = 1000;
        
        // カードを作成
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Heart, 10).unwrap();
        let cards = vec![card1, card2];
        
        let current_bet = 500;
        let is_folded = false;
        let is_all_in = false;
        let is_dealer = true;
        
        // from_serializedでプレイヤーを再構築
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
        
        assert!(result.is_ok());
        let player = result.unwrap();
        
        // 元の値と一致するか確認
        assert_eq!(player.id().value(), player_id.value());
        assert_eq!(player.name(), &name);
        assert_eq!(player.chips(), chips);
        assert_eq!(player.current_bet(), current_bet);
        assert_eq!(player.is_folded(), is_folded);
        assert_eq!(player.is_all_in(), is_all_in);
        assert_eq!(player.is_dealer(), is_dealer);
        
        // 手札の確認
        assert_eq!(player.hand().size(), 2);
        assert_eq!(player.hand().cards()[0].suit(), Suit::Spade);
        assert_eq!(player.hand().cards()[0].rank(), 1);
        assert_eq!(player.hand().cards()[1].suit(), Suit::Heart);
        assert_eq!(player.hand().cards()[1].rank(), 10);
    }
} 