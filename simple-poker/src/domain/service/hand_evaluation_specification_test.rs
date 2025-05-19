#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::card::{Card, Suit};

    fn テスト用手札(cards: Vec<(Suit, u8)>) -> Vec<Card> {
        cards.into_iter()
            .map(|(suit, rank)| Card::new(suit, rank).unwrap())
            .collect()
    }

    #[test]
    fn フラッシュ仕様判定_成功() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 2),
            (Suit::Heart, 5),
            (Suit::Heart, 7),
            (Suit::Heart, 10),
            (Suit::Heart, 13),
        ]);
        
        let spec = FlushSpecification;
        assert!(spec.is_satisfied_by(&hand));
        assert_eq!(spec.name(), "フラッシュ");
    }

    #[test]
    fn フラッシュ仕様判定_失敗() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 2),
            (Suit::Heart, 5),
            (Suit::Heart, 7),
            (Suit::Heart, 10),
            (Suit::Spade, 13), // 1枚だけスートが異なる
        ]);
        
        let spec = FlushSpecification;
        assert!(!spec.is_satisfied_by(&hand));
    }

    #[test]
    fn ストレート仕様判定_通常_成功() {
        let hand = テスト用手札(vec![
            (Suit::Club, 3),
            (Suit::Diamond, 4),
            (Suit::Heart, 5),
            (Suit::Spade, 6),
            (Suit::Club, 7),
        ]);
        
        let spec = StraightSpecification;
        assert!(spec.is_satisfied_by(&hand));
        assert_eq!(spec.name(), "ストレート");
    }

    #[test]
    fn ストレート仕様判定_ホイール_成功() {
        // A-2-3-4-5 のホイール
        let hand = テスト用手札(vec![
            (Suit::Club, 1),
            (Suit::Diamond, 2),
            (Suit::Heart, 3),
            (Suit::Spade, 4),
            (Suit::Club, 5),
        ]);
        
        let spec = StraightSpecification;
        assert!(spec.is_satisfied_by(&hand));
    }

    #[test]
    fn ストレート仕様判定_ブロードウェイ_成功() {
        // 10-J-Q-K-A のブロードウェイ
        let hand = テスト用手札(vec![
            (Suit::Club, 10),
            (Suit::Diamond, 11),
            (Suit::Heart, 12),
            (Suit::Spade, 13),
            (Suit::Club, 1),
        ]);
        
        let spec = StraightSpecification;
        assert!(spec.is_satisfied_by(&hand));
    }

    #[test]
    fn ストレート仕様判定_失敗() {
        let hand = テスト用手札(vec![
            (Suit::Club, 2),
            (Suit::Diamond, 4),
            (Suit::Heart, 6),
            (Suit::Spade, 8),
            (Suit::Club, 10),
        ]);
        
        let spec = StraightSpecification;
        assert!(!spec.is_satisfied_by(&hand));
    }

    #[test]
    fn ロイヤルストレートフラッシュ仕様判定_成功() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 1),
            (Suit::Heart, 10),
            (Suit::Heart, 11),
            (Suit::Heart, 12),
            (Suit::Heart, 13),
        ]);
        
        let spec = RoyalStraightFlushSpecification;
        assert!(spec.is_satisfied_by(&hand));
        assert_eq!(spec.name(), "ロイヤルストレートフラッシュ");
    }

    #[test]
    fn ロイヤルストレートフラッシュ仕様判定_失敗_スートが異なる() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 1),
            (Suit::Heart, 10),
            (Suit::Heart, 11),
            (Suit::Heart, 12),
            (Suit::Spade, 13), // 1枚だけスートが異なる
        ]);
        
        let spec = RoyalStraightFlushSpecification;
        assert!(!spec.is_satisfied_by(&hand));
    }

    #[test]
    fn ロイヤルストレートフラッシュ仕様判定_失敗_数値が異なる() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 1),
            (Suit::Heart, 9), // 10ではなく9
            (Suit::Heart, 11),
            (Suit::Heart, 12),
            (Suit::Heart, 13),
        ]);
        
        let spec = RoyalStraightFlushSpecification;
        assert!(!spec.is_satisfied_by(&hand));
    }

    #[test]
    fn フォーカード仕様判定_成功() {
        let hand = テスト用手札(vec![
            (Suit::Club, 7),
            (Suit::Diamond, 7),
            (Suit::Heart, 7),
            (Suit::Spade, 7),
            (Suit::Club, 10),
        ]);
        
        let spec = FourOfAKindSpecification;
        assert!(spec.is_satisfied_by(&hand));
        assert_eq!(spec.name(), "フォーカード");
    }

    #[test]
    fn フォーカード仕様判定_失敗() {
        let hand = テスト用手札(vec![
            (Suit::Club, 7),
            (Suit::Diamond, 7),
            (Suit::Heart, 7),
            (Suit::Spade, 8), // 異なるランク
            (Suit::Club, 10),
        ]);
        
        let spec = FourOfAKindSpecification;
        assert!(!spec.is_satisfied_by(&hand));
    }

    #[test]
    fn フルハウス仕様判定_成功() {
        let hand = テスト用手札(vec![
            (Suit::Club, 9),
            (Suit::Diamond, 9),
            (Suit::Heart, 9),
            (Suit::Spade, 5),
            (Suit::Club, 5),
        ]);
        
        let spec = FullHouseSpecification;
        assert!(spec.is_satisfied_by(&hand));
        assert_eq!(spec.name(), "フルハウス");
    }

    #[test]
    fn フルハウス仕様判定_失敗() {
        let hand = テスト用手札(vec![
            (Suit::Club, 9),
            (Suit::Diamond, 9),
            (Suit::Heart, 9),
            (Suit::Spade, 5),
            (Suit::Club, 6), // ペアではない
        ]);
        
        let spec = FullHouseSpecification;
        assert!(!spec.is_satisfied_by(&hand));
    }

    #[test]
    fn 複合仕様_AND_成功() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 5),
            (Suit::Heart, 6),
            (Suit::Heart, 7),
            (Suit::Heart, 8),
            (Suit::Heart, 9),
        ]);
        
        let straight_spec = StraightSpecification;
        let flush_spec = FlushSpecification;
        let and_spec = AndSpecification::new(straight_spec, flush_spec, "ストレートフラッシュ");
        
        assert!(and_spec.is_satisfied_by(&hand));
        assert_eq!(and_spec.name(), "ストレートフラッシュ");
    }

    #[test]
    fn 複合仕様_AND_失敗() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 5),
            (Suit::Heart, 6),
            (Suit::Heart, 7),
            (Suit::Spade, 8), // スートが異なる
            (Suit::Heart, 9),
        ]);
        
        let straight_spec = StraightSpecification;
        let flush_spec = FlushSpecification;
        let and_spec = AndSpecification::new(straight_spec, flush_spec, "ストレートフラッシュ");
        
        assert!(!and_spec.is_satisfied_by(&hand));
    }

    #[test]
    fn 複合仕様_OR_成功() {
        let hand = テスト用手札(vec![
            (Suit::Club, 7),
            (Suit::Diamond, 7),
            (Suit::Heart, 7),
            (Suit::Spade, 7),
            (Suit::Club, 10),
        ]);
        
        let four_of_kind_spec = FourOfAKindSpecification;
        let full_house_spec = FullHouseSpecification;
        let or_spec = OrSpecification::new(four_of_kind_spec, full_house_spec, "フォーカードかフルハウス");
        
        assert!(or_spec.is_satisfied_by(&hand));
        assert_eq!(or_spec.name(), "フォーカードかフルハウス");
    }

    #[test]
    fn 複合仕様_OR_両方成立() {
        // このケースはポーカーでは起こりえないが、テストのため
        let hand = テスト用手札(vec![
            (Suit::Heart, 7),
            (Suit::Heart, 7),
            (Suit::Heart, 7),
            (Suit::Heart, 7),
            (Suit::Heart, 7),
        ]);
        
        let four_of_kind_spec = FourOfAKindSpecification;
        let flush_spec = FlushSpecification;
        let or_spec = OrSpecification::new(four_of_kind_spec, flush_spec, "フォーカードかフラッシュ");
        
        assert!(or_spec.is_satisfied_by(&hand));
    }

    #[test]
    fn 複合仕様_NOT_成功() {
        let hand = テスト用手札(vec![
            (Suit::Club, 2),
            (Suit::Diamond, 5),
            (Suit::Heart, 7),
            (Suit::Spade, 9),
            (Suit::Club, 11),
        ]);
        
        let flush_spec = FlushSpecification;
        let not_spec = NotSpecification::new(flush_spec, "フラッシュでない");
        
        assert!(not_spec.is_satisfied_by(&hand));
        assert_eq!(not_spec.name(), "フラッシュでない");
    }

    #[test]
    fn 複合仕様_NOT_失敗() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 2),
            (Suit::Heart, 5),
            (Suit::Heart, 7),
            (Suit::Heart, 9),
            (Suit::Heart, 11),
        ]);
        
        let flush_spec = FlushSpecification;
        let not_spec = NotSpecification::new(flush_spec, "フラッシュでない");
        
        assert!(!not_spec.is_satisfied_by(&hand));
    }
} 