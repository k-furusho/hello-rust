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
    fn ロイヤルストレートフラッシュ判定() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 1),
            (Suit::Spade, 10),
            (Suit::Spade, 11),
            (Suit::Spade, 12),
            (Suit::Spade, 13),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::RoyalStraightFlush);
    }

    #[test]
    fn ストレートフラッシュ判定() {
        let hand = テスト用手札(vec![
            (Suit::Heart, 2),
            (Suit::Heart, 3),
            (Suit::Heart, 4),
            (Suit::Heart, 5),
            (Suit::Heart, 6),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::StraightFlush);
    }

    #[test]
    fn フォーカード判定() {
        let hand = テスト用手札(vec![
            (Suit::Club, 8),
            (Suit::Diamond, 8),
            (Suit::Heart, 8),
            (Suit::Spade, 8),
            (Suit::Club, 10),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::FourOfAKind);
        assert_eq!(strength.values()[0], 8); // フォーカードの値
    }

    #[test]
    fn フルハウス判定() {
        let hand = テスト用手札(vec![
            (Suit::Club, 7),
            (Suit::Diamond, 7),
            (Suit::Heart, 7),
            (Suit::Club, 2),
            (Suit::Diamond, 2),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::FullHouse);
        assert_eq!(strength.values()[0], 7); // スリーカードの値
        assert_eq!(strength.values()[1], 2); // ペアの値
    }

    #[test]
    fn フラッシュ判定() {
        let hand = テスト用手札(vec![
            (Suit::Diamond, 2),
            (Suit::Diamond, 5),
            (Suit::Diamond, 7),
            (Suit::Diamond, 9),
            (Suit::Diamond, 11),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::Flush);
    }

    #[test]
    fn ストレート判定() {
        let hand = テスト用手札(vec![
            (Suit::Club, 4),
            (Suit::Diamond, 5),
            (Suit::Heart, 6),
            (Suit::Spade, 7),
            (Suit::Club, 8),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::Straight);
        assert_eq!(strength.values()[0], 8); // ストレートの最高値
    }

    #[test]
    fn ホイール_ストレート判定() {
        // A-2-3-4-5 のホイール
        let hand = テスト用手札(vec![
            (Suit::Club, 1),
            (Suit::Diamond, 2),
            (Suit::Heart, 3),
            (Suit::Spade, 4),
            (Suit::Club, 5),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::Straight);
        assert_eq!(strength.values()[0], 5); // A-5ストレートの最高値は5
    }

    #[test]
    fn スリーカード判定() {
        let hand = テスト用手札(vec![
            (Suit::Club, 9),
            (Suit::Diamond, 9),
            (Suit::Heart, 9),
            (Suit::Spade, 5),
            (Suit::Club, 2),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::ThreeOfAKind);
        assert_eq!(strength.values()[0], 9); // スリーカードの値
    }

    #[test]
    fn ツーペア判定() {
        let hand = テスト用手札(vec![
            (Suit::Club, 10),
            (Suit::Diamond, 10),
            (Suit::Heart, 6),
            (Suit::Spade, 6),
            (Suit::Club, 3),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::TwoPair);
        assert_eq!(strength.values()[0], 10); // 高い方のペアの値
        assert_eq!(strength.values()[1], 6);  // 低い方のペアの値
    }

    #[test]
    fn ワンペア判定() {
        let hand = テスト用手札(vec![
            (Suit::Club, 13),
            (Suit::Diamond, 13),
            (Suit::Heart, 9),
            (Suit::Spade, 5),
            (Suit::Club, 2),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::OnePair);
        assert_eq!(strength.values()[0], 13); // ペアの値
    }

    #[test]
    fn ハイカード判定() {
        let hand = テスト用手札(vec![
            (Suit::Club, 13),
            (Suit::Diamond, 10),
            (Suit::Heart, 8),
            (Suit::Spade, 7),
            (Suit::Club, 2),
        ]);
        
        let strength = HandEvaluationService::evaluate_hand(&hand);
        assert_eq!(strength.rank(), HandRank::HighCard);
        assert_eq!(strength.values()[0], 13); // 最高カードの値
    }

    #[test]
    fn テキサスホールデム役判定() {
        let hole_cards = テスト用手札(vec![
            (Suit::Heart, 1),  // A♥
            (Suit::Heart, 13), // K♥
        ]);
        
        let community_cards = テスト用手札(vec![
            (Suit::Heart, 12), // Q♥
            (Suit::Heart, 11), // J♥
            (Suit::Heart, 10), // 10♥
            (Suit::Club, 5),   // 5♣
            (Suit::Diamond, 7) // 7♦
        ]);
        
        let strength = HandEvaluationService::find_best_hand(
            &hole_cards, 
            &community_cards, 
            GameVariant::TexasHoldem
        );
        
        assert_eq!(strength.rank(), HandRank::RoyalStraightFlush);
    }

    #[test]
    fn オマハ役判定() {
        let hole_cards = テスト用手札(vec![
            (Suit::Heart, 1),  // A♥
            (Suit::Heart, 13), // K♥
            (Suit::Club, 1),   // A♣
            (Suit::Club, 2),   // 2♣
        ]);
        
        let community_cards = テスト用手札(vec![
            (Suit::Heart, 12), // Q♥
            (Suit::Heart, 11), // J♥
            (Suit::Heart, 10), // 10♥
            (Suit::Spade, 3),  // 3♠
            (Suit::Diamond, 4) // 4♦
        ]);
        
        let strength = HandEvaluationService::find_best_hand(
            &hole_cards, 
            &community_cards, 
            GameVariant::Omaha
        );
        
        // オマハでは2枚のホールカードを使う必要があるので、A♥K♥ + Q♥J♥10♥でロイヤルストレートフラッシュができる
        assert_eq!(strength.rank(), HandRank::RoyalStraightFlush);
    }

    #[test]
    fn 最強の五枚組み探索() {
        // 7枚のカードから最強の5枚を見つける
        let cards = テスト用手札(vec![
            (Suit::Club, 8),
            (Suit::Diamond, 8),
            (Suit::Heart, 8),
            (Suit::Spade, 8),
            (Suit::Club, 10),
            (Suit::Diamond, 10),
            (Suit::Heart, 10),
        ]);
        
        let strength = HandEvaluationService::find_best_five_card_hand(&cards);
        assert_eq!(strength.rank(), HandRank::FourOfAKind);
        assert_eq!(strength.values()[0], 8);  // フォーカードの値
        assert_eq!(strength.values()[1], 10); // キッカーの値
    }
} 