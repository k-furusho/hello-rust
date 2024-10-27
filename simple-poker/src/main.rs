use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;
use std::io::{self};

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

const HAND_SIZE: usize = 5;
const MIN_RANK: i32 = 1;
const MAX_RANK: i32 = 13;
const RANKS: std::ops::RangeInclusive<i32> = MIN_RANK..=MAX_RANK;
const ROYAL_STRAIGHT: [i32; 5] = [1, 10, 11, 12, 13];

fn main() {
    let mut deck = create_deck();
    deck.shuffle(&mut rand::thread_rng());

    let mut hand = draw_hand(&mut deck);
    display_hand(&hand);

    println!("入れ替えたいカードの番号を入力してください(例:1 2 3)");
    let numbers = match get_user_input() {
        Ok(nums) => nums,
        Err(e) => {
            println!("エラー: {}", e);
            return;
        }
    };

    replace_cards(&mut hand, &mut deck, &numbers);
    display_hand(&hand);

    evaluate_hand(&hand);
}

fn create_deck() -> Vec<Card> {
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
    let mut deck = Vec::new();
    for suit in suits {
        for rank in RANKS {
            deck.push(Card { suit, rank });
        }
    }
    deck
}

fn draw_hand(deck: &mut Vec<Card>) -> Vec<Card> {
    let mut hand = Vec::new();
    for _ in 0..HAND_SIZE {
        if let Some(card) = deck.pop() {
            hand.push(card);
        }
    }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    hand
}

fn display_hand(hand: &[Card]) {
    println!("--手札--");
    for (i, card) in hand.iter().enumerate() {
        println!("{}: {}", i + 1, card);
    }
}

fn get_user_input() -> Result<Vec<usize>, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "入力エラー")?;

    let numbers: Vec<usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    if numbers.iter().all(|&n| (1..=HAND_SIZE).contains(&n)) {
        Ok(numbers)
    } else {
        Err(format!("1から{}までの数字を入力してください", HAND_SIZE))
    }
}

fn replace_cards(hand: &mut [Card], deck: &mut Vec<Card>, numbers: &[usize]) {
    for &number in numbers {
        if number < 1 || number > hand.len() {
            println!("無効な番号: {}", number);
            continue;
        }
        if let Some(card) = deck.pop() {
            hand[number - 1] = card;
        } else {
            println!("デッキにカードが足りません");
            break;
        }
    }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
}

fn evaluate_hand(hand: &[Card]) {
    let rank = determine_hand_rank(hand);
    println!("役: {}", rank);
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

    fn create_test_hand(cards: Vec<(Suit, i32)>) -> Vec<Card> {
        cards
            .into_iter()
            .map(|(suit, rank)| Card { suit, rank })
            .collect()
    }

    #[test]
    fn test_royal_straight_flush() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 1),
            (Suit::Spade, 10),
            (Suit::Spade, 11),
            (Suit::Spade, 12),
            (Suit::Spade, 13),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::RoyalStraightFlush);
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 7),
            (Suit::Club, 7),
            (Suit::Spade, 2),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 7),
            (Suit::Club, 2),
            (Suit::Spade, 2),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::FullHouse);
    }

    #[test]
    fn test_high_card() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 2),
            (Suit::Heart, 4),
            (Suit::Diamond, 6),
            (Suit::Club, 8),
            (Suit::Spade, 10),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::HighCard);
    }

    #[test]
    fn test_straight() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 2),
            (Suit::Heart, 3),
            (Suit::Diamond, 4),
            (Suit::Club, 5),
            (Suit::Spade, 6),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::Straight);
    }

    #[test]
    fn test_flush() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 2),
            (Suit::Spade, 4),
            (Suit::Spade, 6),
            (Suit::Spade, 8),
            (Suit::Spade, 10),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::Flush);
    }

    #[test]
    fn test_straight_flush() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 2),
            (Suit::Spade, 3),
            (Suit::Spade, 4),
            (Suit::Spade, 5),
            (Suit::Spade, 6),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::StraightFlush);
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 7),
            (Suit::Club, 2),
            (Suit::Spade, 3),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::ThreeOfAKind);
    }

    #[test]
    fn test_two_pair() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 2),
            (Suit::Club, 2),
            (Suit::Spade, 3),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::TwoPair);
    }

    #[test]
    fn test_one_pair() {
        let hand = create_test_hand(vec![
            (Suit::Spade, 7),
            (Suit::Heart, 7),
            (Suit::Diamond, 2),
            (Suit::Club, 3),
            (Suit::Spade, 4),
        ]);
        assert_eq!(determine_hand_rank(&hand), HandRank::OnePair);
    }
}
