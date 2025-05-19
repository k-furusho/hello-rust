#[cfg(test)]
mod tests {
    use crate::domain::model::card::{Card, Suit};

    #[test]
    fn カード作成_有効なランク() {
        // 有効なランク（1-13）でカードを作成
        let card1 = Card::new(Suit::Spade, 1);
        let card2 = Card::new(Suit::Heart, 13);
        assert!(card1.is_ok(), "スペードAの作成に失敗しました");
        assert!(card2.is_ok(), "ハートKの作成に失敗しました");
        let card1 = card1.unwrap();
        let card2 = card2.unwrap();
        assert_eq!(card1.suit(), Suit::Spade, "スートが一致しません");
        assert_eq!(card1.rank(), 1, "ランクが一致しません");
        assert_eq!(card2.suit(), Suit::Heart, "スートが一致しません");
        assert_eq!(card2.rank(), 13, "ランクが一致しません");
    }
    
    #[test]
    fn カード作成_無効なランク() {
        // 無効なランク（0や14以上）でカードを作成
        let card1 = Card::new(Suit::Spade, 0);
        let card2 = Card::new(Suit::Heart, 14);
        assert!(card1.is_err(), "ランク0でエラーになりません");
        assert!(card2.is_err(), "ランク14でエラーになりません");
    }
    
    #[test]
    fn エース判定() {
        let ace = Card::new(Suit::Spade, 1).unwrap();
        let king = Card::new(Suit::Heart, 13).unwrap();
        assert!(ace.is_ace(), "Aがエースと判定されません");
        assert!(!king.is_ace(), "Kがエースと判定されています");
    }
    
    #[test]
    fn フェイスカード判定() {
        let jack = Card::new(Suit::Spade, 11).unwrap();
        let queen = Card::new(Suit::Heart, 12).unwrap();
        let king = Card::new(Suit::Diamond, 13).unwrap();
        let ace = Card::new(Suit::Club, 1).unwrap();
        let ten = Card::new(Suit::Spade, 10).unwrap();
        assert!(jack.is_face_card(), "Jがフェイスカードと判定されません");
        assert!(queen.is_face_card(), "Qがフェイスカードと判定されません");
        assert!(king.is_face_card(), "Kがフェイスカードと判定されません");
        assert!(!ace.is_face_card(), "Aがフェイスカードと判定されています");
        assert!(!ten.is_face_card(), "10がフェイスカードと判定されています");
    }
    
    #[test]
    fn カード等価性() {
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Spade, 1).unwrap();
        let card3 = Card::new(Suit::Heart, 1).unwrap();
        assert_eq!(card1, card2, "同じスート・ランクのカードが等価でありません");
        assert_ne!(card1, card3, "異なるスートのカードが等価と判定されています");
    }
    
    #[test]
    fn カード表示() {
        let ace_spades = Card::new(Suit::Spade, 1).unwrap();
        let ten_hearts = Card::new(Suit::Heart, 10).unwrap();
        let jack_clubs = Card::new(Suit::Club, 11).unwrap();
        let queen_diamonds = Card::new(Suit::Diamond, 12).unwrap();
        let king_spades = Card::new(Suit::Spade, 13).unwrap();
        assert_eq!(format!("{}", ace_spades), "♠A", "Aの表示が正しくありません");
        assert_eq!(format!("{}", ten_hearts), "♥10", "10の表示が正しくありません");
        assert_eq!(format!("{}", jack_clubs), "♣J", "Jの表示が正しくありません");
        assert_eq!(format!("{}", queen_diamonds), "♦Q", "Qの表示が正しくありません");
        assert_eq!(format!("{}", king_spades), "♠K", "Kの表示が正しくありません");
    }
    
    #[test]
    fn スート表示() {
        assert_eq!(format!("{}", Suit::Club), "♣", "クラブの表示が正しくありません");
        assert_eq!(format!("{}", Suit::Diamond), "♦", "ダイヤの表示が正しくありません");
        assert_eq!(format!("{}", Suit::Heart), "♥", "ハートの表示が正しくありません");
        assert_eq!(format!("{}", Suit::Spade), "♠", "スペードの表示が正しくありません");
    }
} 