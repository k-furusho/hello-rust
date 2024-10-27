use rand::seq::SliceRandom;
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

const HAND_SIZE: usize = 5;
const RANKS: std::ops::RangeInclusive<i32> = 1..=13;

fn main() {
    let mut deck = create_deck();
    deck.shuffle(&mut rand::thread_rng());

    let mut hand = draw_hand(&mut deck);
    display_hand(&hand);

    println!("入れ替えたいカードの番号を入力してください(例:1 2 3)");
    let numbers = get_user_input();

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
    println!("--Hand--");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {}", i + 1, card.suit, rank_to_string(card.rank));
    }
}

fn get_user_input() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn replace_cards(hand: &mut Vec<Card>, deck: &mut Vec<Card>, numbers: &[usize]) {
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
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);

    let mut count = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("フラッシュ！");
    } else if count >= 3 {
        println!("スリーカード！");
    } else if count == 2 {
        println!("2ペア!!");
    } else if count == 1 {
        println!("1ペア!!");
    } else {
        println!("約なし...");
    }
}

fn rank_to_string(rank: i32) -> String {
    match rank {
        1 => "A".to_string(),
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        _ => rank.to_string(),
    }
}
