#[cfg(test)]
mod tests {
    use crate::domain::model::deck::Deck;
    use crate::domain::model::card::{Card, Suit};
    use std::collections::HashSet;

    #[test]
    fn デッキ作成() {
        let deck = Deck::new();
        assert!(deck.is_ok(), "デッキの作成に失敗しました");
        let deck = deck.unwrap();
        assert_eq!(deck.remaining(), 52, "デッキの初期枚数が52ではありません");
        // すべてのカードが一意であるか
        let mut deck_for_uniqueness = Deck::new().unwrap();
        let mut unique_cards = HashSet::new();
        while let Some(card) = deck_for_uniqueness.draw() {
            assert!(unique_cards.insert((card.suit(), card.rank())), "カードが重複しています");
        }
        assert_eq!(unique_cards.len(), 52, "デッキ内のカード種類が52ではありません");
    }

    #[test]
    fn デッキシャッフル() {
        let mut deck1 = Deck::new().unwrap();
        let mut deck2 = Deck::new().unwrap();
        let cards1: Vec<Card> = (0..10).filter_map(|_| deck1.draw()).collect();
        let cards2: Vec<Card> = (0..10).filter_map(|_| deck2.draw()).collect();
        assert_eq!(cards1.len(), cards2.len(), "カード枚数が一致しません");
        for i in 0..cards1.len() {
            assert_eq!(cards1[i].suit(), cards2[i].suit(), "シャッフル前のスートが一致しません");
            assert_eq!(cards1[i].rank(), cards2[i].rank(), "シャッフル前のランクが一致しません");
        }
        let mut deck1 = Deck::new().unwrap();
        let mut deck2 = Deck::new().unwrap();
        deck1.shuffle();
        let cards1: Vec<Card> = (0..10).filter_map(|_| deck1.draw()).collect();
        let cards2: Vec<Card> = (0..10).filter_map(|_| deck2.draw()).collect();
        let mut all_equal = true;
        for i in 0..cards1.len() {
            if cards1[i].suit() != cards2[i].suit() || cards1[i].rank() != cards2[i].rank() {
                all_equal = false;
                break;
            }
        }
        assert!(!all_equal, "シャッフル後もカードの順序が同じです（まれに起こる可能性があります）");
    }

    #[test]
    fn デッキからカードを引く() {
        let mut deck = Deck::new().unwrap();
        let initial_count = deck.remaining();
        let card = deck.draw();
        assert!(card.is_some(), "カードを引けませんでした");
        assert_eq!(deck.remaining(), initial_count - 1, "カードを引いた後の残り枚数が一致しません");
        for _ in 0..initial_count - 1 {
            let _ = deck.draw();
        }
        assert_eq!(deck.remaining(), 0, "全て引いた後もデッキが空ではありません");
        assert!(deck.is_empty(), "デッキが空でないと判定されています");
        let card = deck.draw();
        assert!(card.is_none(), "空のデッキからカードが引けてしまいます");
    }

    #[test]
    fn デッキから複数枚引く() {
        let mut deck = Deck::new().unwrap();
        let initial_count = deck.remaining();
        let cards = deck.draw_multiple(5);
        assert_eq!(cards.len(), 5, "5枚引けませんでした");
        assert_eq!(deck.remaining(), initial_count - 5, "5枚引いた後の残り枚数が一致しません");
        let remaining = deck.remaining();
        let cards = deck.draw_multiple(remaining + 10);
        assert_eq!(cards.len(), remaining, "残り以上のカードが引けています");
        assert_eq!(deck.remaining(), 0, "全て引いた後もデッキが空ではありません");
    }

    #[test]
    fn デッキにカードを追加() {
        let mut deck = Deck::new().unwrap();
        let initial_count = deck.remaining();
        let new_card = Card::new(Suit::Spade, 1).unwrap();
        deck.add_card(new_card);
        assert_eq!(deck.remaining(), initial_count + 1, "カード追加後のデッキ枚数が一致しません");
    }

    #[test]
    fn デッキ残数と空判定() {
        let mut deck = Deck::new().unwrap();
        assert_eq!(deck.remaining(), 52, "初期デッキ枚数が52ではありません");
        assert!(!deck.is_empty(), "初期状態でデッキが空と判定されています");
        for _ in 0..52 {
            let _ = deck.draw();
        }
        assert_eq!(deck.remaining(), 0, "全て引いた後もデッキが空ではありません");
        assert!(deck.is_empty(), "全て引いた後にデッキが空でないと判定されています");
    }
} 