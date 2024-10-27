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
    let suit = Suit::Club;
    let rank = 1;
    let card = Card { suit, rank };
    println!("{:?}", card);
}
