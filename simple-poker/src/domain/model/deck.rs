use rand::seq::SliceRandom;

use super::card::{Card, Suit};

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Result<Self, &'static str> {
        let mut cards = Vec::with_capacity(52);
        
        for &suit in Suit::all().iter() {
            for rank in 1..=13 {
                if let Ok(card) = Card::new(suit, rank) {
                    cards.push(card);
                } else {
                    return Err("デッキの初期化中にエラーが発生しました");
                }
            }
        }
        
        Ok(Self { cards })
    }
    
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }
    
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    
    pub fn draw_multiple(&mut self, count: usize) -> Vec<Card> {
        let mut drawn = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.draw() {
                drawn.push(card);
            } else {
                break;
            }
        }
        drawn
    }
    
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
} 