use std::fmt;
use super::card::Card;

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    max_size: usize,
}

impl Hand {
    pub fn new(max_size: usize) -> Self {
        Self {
            cards: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn add_card(&mut self, card: Card) -> Result<(), &'static str> {
        if self.cards.len() >= self.max_size {
            return Err("手札がいっぱいです");
        }
        self.cards.push(card);
        Ok(())
    }
    
    pub fn replace_card(&mut self, index: usize, new_card: Card) -> Result<Card, &'static str> {
        if index >= self.cards.len() {
            return Err("無効なインデックスです");
        }
        
        Ok(std::mem::replace(&mut self.cards[index], new_card))
    }
    
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
    
    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_size
    }
    
    pub fn size(&self) -> usize {
        self.cards.len()
    }
    
    pub fn max_size(&self) -> usize {
        self.max_size
    }
    
    pub fn clear(&mut self) {
        self.cards.clear();
    }
    
    pub fn sort_by_rank(&mut self) {
        self.cards.sort_by_key(|card| card.rank());
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "手札: ")?;
        for (i, card) in self.cards.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", card)?;
        }
        Ok(())
    }
} 