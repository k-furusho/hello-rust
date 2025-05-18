#[cfg(test)]
mod tests {
    use crate::domain::model::card::{Card, Suit};

    #[test]
    fn test_card_creation_with_valid_rank() {
        // 有効なランク（1-13）でカードを作成
        let card1 = Card::new(Suit::Spade, 1);
        let card2 = Card::new(Suit::Heart, 13);
        
        assert!(card1.is_ok());
        assert!(card2.is_ok());
        
        let card1 = card1.unwrap();
        let card2 = card2.unwrap();
        
        assert_eq!(card1.suit(), Suit::Spade);
        assert_eq!(card1.rank(), 1);
        assert_eq!(card2.suit(), Suit::Heart);
        assert_eq!(card2.rank(), 13);
    }
    
    #[test]
    fn test_card_creation_with_invalid_rank() {
        // 無効なランク（0や14以上）でカードを作成
        let card1 = Card::new(Suit::Spade, 0);
        let card2 = Card::new(Suit::Heart, 14);
        
        assert!(card1.is_err());
        assert!(card2.is_err());
    }
    
    #[test]
    fn test_card_is_ace() {
        let ace = Card::new(Suit::Spade, 1).unwrap();
        let king = Card::new(Suit::Heart, 13).unwrap();
        
        assert!(ace.is_ace());
        assert!(!king.is_ace());
    }
    
    #[test]
    fn test_card_is_face_card() {
        let jack = Card::new(Suit::Spade, 11).unwrap();
        let queen = Card::new(Suit::Heart, 12).unwrap();
        let king = Card::new(Suit::Diamond, 13).unwrap();
        let ace = Card::new(Suit::Club, 1).unwrap();
        let ten = Card::new(Suit::Spade, 10).unwrap();
        
        assert!(jack.is_face_card());
        assert!(queen.is_face_card());
        assert!(king.is_face_card());
        assert!(!ace.is_face_card());
        assert!(!ten.is_face_card());
    }
    
    #[test]
    fn test_card_equality() {
        let card1 = Card::new(Suit::Spade, 1).unwrap();
        let card2 = Card::new(Suit::Spade, 1).unwrap();
        let card3 = Card::new(Suit::Heart, 1).unwrap();
        
        assert_eq!(card1, card2);
        assert_ne!(card1, card3);
    }
    
    #[test]
    fn test_card_display() {
        let ace_spades = Card::new(Suit::Spade, 1).unwrap();
        let ten_hearts = Card::new(Suit::Heart, 10).unwrap();
        let jack_clubs = Card::new(Suit::Club, 11).unwrap();
        let queen_diamonds = Card::new(Suit::Diamond, 12).unwrap();
        let king_spades = Card::new(Suit::Spade, 13).unwrap();
        
        assert_eq!(format!("{}", ace_spades), "♠A");
        assert_eq!(format!("{}", ten_hearts), "♥10");
        assert_eq!(format!("{}", jack_clubs), "♣J");
        assert_eq!(format!("{}", queen_diamonds), "♦Q");
        assert_eq!(format!("{}", king_spades), "♠K");
    }
    
    #[test]
    fn test_suit_display() {
        assert_eq!(format!("{}", Suit::Club), "♣");
        assert_eq!(format!("{}", Suit::Diamond), "♦");
        assert_eq!(format!("{}", Suit::Heart), "♥");
        assert_eq!(format!("{}", Suit::Spade), "♠");
    }
} 