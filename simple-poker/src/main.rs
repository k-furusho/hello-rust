use std::fmt;
use std::collections::HashMap;
use simple_poker::presentation::cli::menu::MenuController;
use simple_poker::infrastructure::repository::inmemory::game_repository_inmemory::InMemoryGameRepository;
use simple_poker::infrastructure::repository::inmemory::player_repository_inmemory::InMemoryPlayerRepository;
use simple_poker::infrastructure::service::event::inmemory_event_publisher::InMemoryEventPublisher;

const ROYAL_STRAIGHT: [i32; 5] = [1, 10, 11, 12, 13];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_str = match self.suit {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        let rank_str = match self.rank {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n => n.to_string(),
        };
        write!(f, "{}{}", suit_str, rank_str)
    }
}

#[derive(Debug, PartialEq)]
enum HandRank {
    RoyalStraightFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self {
            HandRank::RoyalStraightFlush => "ロイヤルストレートフラッシュ",
            HandRank::StraightFlush => "ストレートフラッシュ",
            HandRank::FourOfAKind => "フォーカード",
            HandRank::FullHouse => "フルハウス",
            HandRank::Flush => "フラッシュ",
            HandRank::Straight => "ストレート",
            HandRank::ThreeOfAKind => "スリーカード",
            HandRank::TwoPair => "ツーペア",
            HandRank::OnePair => "ワンペア",
            HandRank::HighCard => "ハイカード",
        };
        write!(f, "{}", rank_str)
    }
}

fn main() {
    // リポジトリの初期化
    let game_repository = InMemoryGameRepository::new();
    let player_repository = InMemoryPlayerRepository::new();
    let event_publisher = InMemoryEventPublisher::new();
    
    // メニューコントローラの作成と実行
    let mut menu = MenuController::new(game_repository, player_repository, event_publisher);
    menu.run();
}

fn determine_hand_rank(hand: &[Card]) -> HandRank {
    let is_flush = is_flush(hand);
    let is_straight = is_straight(hand);
    let is_royal = is_royal(hand);
    let rank_counts = count_ranks(hand);

    match (is_flush, is_straight, is_royal) {
        (true, true, true) => HandRank::RoyalStraightFlush,
        (true, true, false) => HandRank::StraightFlush,
        (true, false, _) => HandRank::Flush,
        (false, true, _) => HandRank::Straight,
        _ => determine_rank_by_counts(&rank_counts),
    }
}

fn is_flush(hand: &[Card]) -> bool {
    let first_suit = hand[0].suit;
    hand.iter().all(|card| card.suit == first_suit)
}

fn is_straight(hand: &[Card]) -> bool {
    let mut ranks: Vec<i32> = hand.iter().map(|c| c.rank).collect();
    ranks.sort();

    if ranks == ROYAL_STRAIGHT {
        return true;
    }

    ranks.windows(2).all(|w| w[1] == w[0] + 1)
}

fn is_royal(hand: &[Card]) -> bool {
    let mut ranks: Vec<i32> = hand.iter().map(|c| c.rank).collect();
    ranks.sort();
    ranks == vec![1, 10, 11, 12, 13]
}

fn count_ranks(hand: &[Card]) -> HashMap<i32, i32> {
    let mut rank_counts = HashMap::new();
    for card in hand {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }
    rank_counts
}

fn determine_rank_by_counts(rank_counts: &HashMap<i32, i32>) -> HandRank {
    let max_count = rank_counts.values().max().unwrap();
    let pair_count = rank_counts.values().filter(|&&count| count == 2).count();

    match *max_count {
        4 => HandRank::FourOfAKind,
        3 => {
            if pair_count == 1 {
                HandRank::FullHouse
            } else {
                HandRank::ThreeOfAKind
            }
        }
        2 => {
            if pair_count == 2 {
                HandRank::TwoPair
            } else {
                HandRank::OnePair
            }
        }
        _ => HandRank::HighCard,
    }
}

// テストモジュールの追加
#[cfg(test)]
mod tests {
    use super::*;

    fn テスト用手札(cards: Vec<(Suit, i32)>) -> Vec<Card> {
        cards.into_iter().map(|(suit, rank)| Card { suit, rank }).collect()
    }

    #[test]
    fn ロイヤルストレートフラッシュ() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 1),
            (Suit::Spade, 10),
            (Suit::Spade, 11),
            (Suit::Spade, 12),
            (Suit::Spade, 13),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::RoyalStraightFlush, "ロイヤルストレートフラッシュ判定失敗");
    }

    #[test]
    fn フォーカード() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 7),
            (Suit::Club, 7),
            (Suit::Spade, 2),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::FourOfAKind, "フォーカード判定失敗");
    }

    #[test]
    fn フルハウス() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 7),
            (Suit::Club, 2),
            (Suit::Spade, 2),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::FullHouse, "フルハウス判定失敗");
    }

    #[test]
    fn ハイカード() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 2),
            (Suit::Heart, 4),
            (Suit::Diamond, 6),
            (Suit::Club, 8),
            (Suit::Spade, 10),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::HighCard, "ハイカード判定失敗");
    }

    #[test]
    fn ストレート() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 2),
            (Suit::Heart, 3),
            (Suit::Diamond, 4),
            (Suit::Club, 5),
            (Suit::Spade, 6),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::Straight, "ストレート判定失敗");
    }

    #[test]
    fn フラッシュ() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 2),
            (Suit::Spade, 4),
            (Suit::Spade, 6),
            (Suit::Spade, 8),
            (Suit::Spade, 10),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::Flush, "フラッシュ判定失敗");
    }

    #[test]
    fn ストレートフラッシュ() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 2),
            (Suit::Spade, 3),
            (Suit::Spade, 4),
            (Suit::Spade, 5),
            (Suit::Spade, 6),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::StraightFlush, "ストレートフラッシュ判定失敗");
    }

    #[test]
    fn スリーカード() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 7),
            (Suit::Club, 2),
            (Suit::Spade, 3),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::ThreeOfAKind, "スリーカード判定失敗");
    }

    #[test]
    fn ツーペア() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 2),
            (Suit::Club, 2),
            (Suit::Spade, 3),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::TwoPair, "ツーペア判定失敗");
    }

    #[test]
    fn ワンペア() {
        let hand = テスト用手札(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 2),
            (Suit::Club, 3),
            (Suit::Spade, 4),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::OnePair, "ワンペア判定失敗");
    }
}
