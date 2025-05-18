#[cfg(test)]
mod tests {
    use crate::domain::model::hand::Hand;
    use crate::domain::model::card::{Card, Suit};

    #[test]
    fn test_hand_creation() {
        let hand = Hand::new(5);
        
        assert_eq!(hand.size(), 0);
        assert_eq!(hand.max_size(), 5);
        assert!(hand.is_empty());
        assert!(!hand.is_full());
    }
    
    #[test]
    fn test_add_card() {
        let mut hand = Hand::new(5);
        let card = Card::new(Suit::Spade, 1).unwrap();
        
        let result = hand.add_card(card);
        assert!(result.is_ok());
        assert_eq!(hand.size(), 1);
        assert!(!hand.is_empty());
        
        // カードが実際に追加されたか確認
        assert_eq!(hand.cards()[0].suit(), Suit::Spade);
        assert_eq!(hand.cards()[0].rank(), 1);
    }
    
    #[test]
    fn test_add_card_to_full_hand() {
        let mut hand = Hand::new(2);
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Heart, 2).unwrap();
        let card3 = Card::new(Suit::Diamond, 3).unwrap();
        
        // 2枚まで追加可能
        let _ = hand.add_card(card1);
        let _ = hand.add_card(card2);
        assert_eq!(hand.size(), 2);
        assert!(hand.is_full());
        
        // 3枚目は追加不可
        let result = hand.add_card(card3);
        assert!(result.is_err());
        assert_eq!(hand.size(), 2); // サイズは変わらない
    }
    
    #[test]
    fn test_replace_card() {
        let mut hand = Hand::new(5);
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Heart, 2).unwrap();
        
        // まずカードを追加
        let _ = hand.add_card(card1);
        assert_eq!(hand.size(), 1);
        
        // カードを交換
        let result = hand.replace_card(0, card2);
        assert!(result.is_ok());
        let old_card = result.unwrap();
        
        // 交換されたカードが正しいか確認
        assert_eq!(old_card.suit(), Suit::Spade);
        assert_eq!(old_card.rank(), 1);
        
        // 新しいカードがセットされたか確認
        assert_eq!(hand.cards()[0].suit(), Suit::Heart);
        assert_eq!(hand.cards()[0].rank(), 2);
    }
    
    #[test]
    fn test_replace_card_invalid_index() {
        let mut hand = Hand::new(5);
        let card = Card::new(Suit::Spade, 1).unwrap();
        
        // 空の手札で無効なインデックスを指定
        let result = hand.replace_card(0, card);
        assert!(result.is_err());
        
        // カードを1枚追加
        let _ = hand.add_card(Card::new(Suit::Heart, 2).unwrap());
        
        // 範囲外のインデックスを指定
        let result = hand.replace_card(1, card);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_clear() {
        let mut hand = Hand::new(5);
        
        // カードをいくつか追加
        let _ = hand.add_card(Card::new(Suit::Spade, 1).unwrap());
        let _ = hand.add_card(Card::new(Suit::Heart, 2).unwrap());
        assert_eq!(hand.size(), 2);
        
        // 手札をクリア
        hand.clear();
        assert_eq!(hand.size(), 0);
        assert!(hand.is_empty());
    }
    
    #[test]
    fn test_sort_by_rank() {
        let mut hand = Hand::new(5);
        
        // ランクがバラバラな順でカードを追加
        let _ = hand.add_card(Card::new(Suit::Spade, 10).unwrap());
        let _ = hand.add_card(Card::new(Suit::Heart, 1).unwrap()); // エース
        let _ = hand.add_card(Card::new(Suit::Diamond, 5).unwrap());
        let _ = hand.add_card(Card::new(Suit::Club, 13).unwrap()); // キング
        
        // ランクでソート
        hand.sort_by_rank();
        
        // ソート後の順序を確認（昇順）
        assert_eq!(hand.cards()[0].rank(), 1);
        assert_eq!(hand.cards()[1].rank(), 5);
        assert_eq!(hand.cards()[2].rank(), 10);
        assert_eq!(hand.cards()[3].rank(), 13);
    }
    
    #[test]
    fn test_is_full_and_is_empty() {
        let mut hand = Hand::new(3);
        
        // 初期状態
        assert!(hand.is_empty());
        assert!(!hand.is_full());
        
        // 1枚追加
        let _ = hand.add_card(Card::new(Suit::Spade, 1).unwrap());
        assert!(!hand.is_empty());
        assert!(!hand.is_full());
        
        // 最大数まで追加
        let _ = hand.add_card(Card::new(Suit::Heart, 2).unwrap());
        let _ = hand.add_card(Card::new(Suit::Diamond, 3).unwrap());
        assert!(!hand.is_empty());
        assert!(hand.is_full());
        
        // クリア
        hand.clear();
        assert!(hand.is_empty());
        assert!(!hand.is_full());
    }
} 