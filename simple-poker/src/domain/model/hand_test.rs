#[cfg(test)]
mod tests {
    use crate::domain::model::hand::Hand;
    use crate::domain::model::card::{Card, Suit};

    #[test]
    fn 手札作成() {
        let hand = Hand::new(5);
        assert_eq!(hand.size(), 0, "初期手札サイズが0ではありません");
        assert_eq!(hand.max_size(), 5, "最大手札サイズが5ではありません");
        assert!(hand.is_empty(), "初期状態で手札が空でない");
        assert!(!hand.is_full(), "初期状態で手札が満杯");
    }

    #[test]
    fn カード追加() {
        let mut hand = Hand::new(5);
        let card = Card::new(Suit::Spade, 1).unwrap();
        let result = hand.add_card(card);
        assert!(result.is_ok(), "カード追加に失敗しました");
        assert_eq!(hand.size(), 1, "カード追加後の手札サイズが一致しません");
        assert!(!hand.is_empty(), "カード追加後も手札が空");
        assert_eq!(hand.cards()[0].suit(), Suit::Spade, "追加したカードのスートが一致しません");
        assert_eq!(hand.cards()[0].rank(), 1, "追加したカードのランクが一致しません");
    }

    #[test]
    fn 手札満杯時の追加() {
        let mut hand = Hand::new(2);
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Heart, 2).unwrap();
        let card3 = Card::new(Suit::Diamond, 3).unwrap();
        let _ = hand.add_card(card1);
        let _ = hand.add_card(card2);
        assert_eq!(hand.size(), 2, "2枚追加後の手札サイズが一致しません");
        assert!(hand.is_full(), "2枚追加後も手札が満杯でない");
        let result = hand.add_card(card3);
        assert!(result.is_err(), "満杯の手札にカードが追加できてしまいます");
        assert_eq!(hand.size(), 2, "満杯時に手札サイズが変化しています");
    }

    #[test]
    fn カード交換() {
        let mut hand = Hand::new(5);
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Heart, 2).unwrap();
        let _ = hand.add_card(card1);
        assert_eq!(hand.size(), 1, "カード追加後の手札サイズが一致しません");
        let result = hand.replace_card(0, card2);
        assert!(result.is_ok(), "カード交換に失敗しました");
        let old_card = result.unwrap();
        assert_eq!(old_card.suit(), Suit::Spade, "交換前カードのスートが一致しません");
        assert_eq!(old_card.rank(), 1, "交換前カードのランクが一致しません");
        assert_eq!(hand.cards()[0].suit(), Suit::Heart, "交換後カードのスートが一致しません");
        assert_eq!(hand.cards()[0].rank(), 2, "交換後カードのランクが一致しません");
    }

    #[test]
    fn カード交換_無効なインデックス() {
        let mut hand = Hand::new(5);
        let card = Card::new(Suit::Spade, 1).unwrap();
        let result = hand.replace_card(0, card);
        assert!(result.is_err(), "空の手札で無効なインデックスでもエラーになりません");
        let _ = hand.add_card(Card::new(Suit::Heart, 2).unwrap());
        let result = hand.replace_card(1, card);
        assert!(result.is_err(), "範囲外インデックスでもエラーになりません");
    }

    #[test]
    fn 手札クリア() {
        let mut hand = Hand::new(5);
        let _ = hand.add_card(Card::new(Suit::Spade, 1).unwrap());
        let _ = hand.add_card(Card::new(Suit::Heart, 2).unwrap());
        assert_eq!(hand.size(), 2, "カード追加後の手札サイズが一致しません");
        hand.clear();
        assert_eq!(hand.size(), 0, "クリア後の手札サイズが0ではありません");
        assert!(hand.is_empty(), "クリア後も手札が空でない");
    }

    #[test]
    fn ランクでソート() {
        let mut hand = Hand::new(5);
        let _ = hand.add_card(Card::new(Suit::Spade, 10).unwrap());
        let _ = hand.add_card(Card::new(Suit::Heart, 1).unwrap());
        let _ = hand.add_card(Card::new(Suit::Diamond, 5).unwrap());
        let _ = hand.add_card(Card::new(Suit::Club, 13).unwrap());
        hand.sort_by_rank();
        assert_eq!(hand.cards()[0].rank(), 1, "ソート後の1枚目がAでない");
        assert_eq!(hand.cards()[1].rank(), 5, "ソート後の2枚目が5でない");
        assert_eq!(hand.cards()[2].rank(), 10, "ソート後の3枚目が10でない");
        assert_eq!(hand.cards()[3].rank(), 13, "ソート後の4枚目がKでない");
    }

    #[test]
    fn 手札満杯判定と空判定() {
        let mut hand = Hand::new(3);
        assert!(hand.is_empty(), "初期状態で手札が空でない");
        assert!(!hand.is_full(), "初期状態で手札が満杯");
        let _ = hand.add_card(Card::new(Suit::Spade, 1).unwrap());
        assert!(!hand.is_empty(), "1枚追加後も手札が空");
        assert!(!hand.is_full(), "1枚追加後も手札が満杯");
        let _ = hand.add_card(Card::new(Suit::Heart, 2).unwrap());
        let _ = hand.add_card(Card::new(Suit::Diamond, 3).unwrap());
        assert!(!hand.is_empty(), "3枚追加後も手札が空");
        assert!(hand.is_full(), "3枚追加後も手札が満杯でない");
        hand.clear();
        assert!(hand.is_empty(), "クリア後も手札が空でない");
        assert!(!hand.is_full(), "クリア後も手札が満杯");
    }
} 