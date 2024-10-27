use rand::seq::SliceRandom;

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

fn main() {
    // Vecの用意
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deckを作成
    for suit in suits {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }
    // Deckをシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // 手札用のVecの用意
    let mut hand: Vec<Card> = Vec::new();
    // 5枚のカードを引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }
    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    //手札を表示
    println!("--Hand--");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    println!("入れ替えたいカードの番号を入力してください(例:1 2 3)");
    // ユーザーからの入力を入れるための変数
    let mut input = String::new();
    // ユーザーからの入力を変数に書き込む
    std::io::stdin().read_line(&mut input).unwrap();
}
