#[cfg(test)]
mod tests {
    use crate::domain::model::deck::Deck;
    use crate::domain::model::card::{Card, Suit};
    use std::collections::HashSet;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert!(deck.is_ok());
        
        let deck = deck.unwrap();
        
        // デッキには52枚のカードが含まれているか
        assert_eq!(deck.remaining(), 52);
        
        // すべてのカードを引いて一意性を確認（破壊的操作）
        let mut deck_for_uniqueness = Deck::new().unwrap();
        let mut unique_cards = HashSet::new();
        
        // デッキからすべてのカードを引き、一意であることを確認
        while let Some(card) = deck_for_uniqueness.draw() {
            assert!(unique_cards.insert((card.suit(), card.rank())));
        }
        
        // 全種類のカードが含まれているか確認
        assert_eq!(unique_cards.len(), 52);
    }
    
    #[test]
    fn test_deck_shuffle() {
        let mut deck1 = Deck::new().unwrap();
        let mut deck2 = Deck::new().unwrap();
        
        // シャッフル前後で確実に違いを検出するため、カードを数枚引いて内容を比較
        let cards1: Vec<Card> = (0..10).filter_map(|_| deck1.draw()).collect();
        let cards2: Vec<Card> = (0..10).filter_map(|_| deck2.draw()).collect();
        
        // シャッフル前は同じ順序（同じカードが出る）
        assert_eq!(cards1.len(), cards2.len());
        for i in 0..cards1.len() {
            assert_eq!(cards1[i].suit(), cards2[i].suit());
            assert_eq!(cards1[i].rank(), cards2[i].rank());
        }
        
        // デッキを再作成してシャッフル
        let mut deck1 = Deck::new().unwrap();
        let mut deck2 = Deck::new().unwrap();
        deck1.shuffle();
        
        // シャッフル後は順序が変わっている可能性が高い
        let cards1: Vec<Card> = (0..10).filter_map(|_| deck1.draw()).collect();
        let cards2: Vec<Card> = (0..10).filter_map(|_| deck2.draw()).collect();
        
        // 確率的なテストなので、完全な一致はまれ
        let mut all_equal = true;
        for i in 0..cards1.len() {
            if cards1[i].suit() != cards2[i].suit() || cards1[i].rank() != cards2[i].rank() {
                all_equal = false;
                break;
            }
        }
        
        // すべてのカードが同じであることはまれなので、通常は異なるはず
        // 注意: 確率的なテストなので、まれに失敗する可能性があります
        assert!(!all_equal, "シャッフル後もカードの順序が同じです（まれに起こる可能性があります）");
    }
    
    #[test]
    fn test_deck_draw() {
        let mut deck = Deck::new().unwrap();
        let initial_count = deck.remaining();
        
        // カードを1枚引く
        let card = deck.draw();
        assert!(card.is_some());
        assert_eq!(deck.remaining(), initial_count - 1);
        
        // デッキを空にする
        for _ in 0..initial_count - 1 {
            let _ = deck.draw();
        }
        
        // デッキが空になったこと
        assert_eq!(deck.remaining(), 0);
        assert!(deck.is_empty());
        
        // 空のデッキからカードを引こうとする
        let card = deck.draw();
        assert!(card.is_none());
    }
    
    #[test]
    fn test_deck_draw_multiple() {
        let mut deck = Deck::new().unwrap();
        let initial_count = deck.remaining();
        
        // 複数枚のカードを引く
        let cards = deck.draw_multiple(5);
        assert_eq!(cards.len(), 5);
        assert_eq!(deck.remaining(), initial_count - 5);
        
        // 残りのカード数以上を引こうとする
        let remaining = deck.remaining();
        let cards = deck.draw_multiple(remaining + 10);
        assert_eq!(cards.len(), remaining); // 残っているだけ引ける
        assert_eq!(deck.remaining(), 0);
    }
    
    #[test]
    fn test_deck_add_card() {
        let mut deck = Deck::new().unwrap();
        let initial_count = deck.remaining();
        
        // カードを1枚追加
        let new_card = Card::new(Suit::Spade, 1).unwrap();
        deck.add_card(new_card);
        
        assert_eq!(deck.remaining(), initial_count + 1);
    }
    
    #[test]
    fn test_deck_remaining_and_is_empty() {
        let mut deck = Deck::new().unwrap();
        assert_eq!(deck.remaining(), 52);
        assert!(!deck.is_empty());
        
        // すべてのカードを引く
        for _ in 0..52 {
            let _ = deck.draw();
        }
        
        assert_eq!(deck.remaining(), 0);
        assert!(deck.is_empty());
    }
} 