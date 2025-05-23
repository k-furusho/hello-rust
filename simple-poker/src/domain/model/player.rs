use std::fmt;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::bet::Chips;
use super::hand::Hand;
use super::card::Card;
use super::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerId(String);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    pub fn from_string(id: String) -> Self {
        Self(id)
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerId,
    name: String,
    hand: Hand,
    chips: Chips,
    current_bet: u32,
    is_folded: bool,
    is_all_in: bool,
    is_dealer: bool,
}

/// デシリアライズのためのデータ構造体
pub struct PlayerSerializedData {
    pub id: PlayerId,
    pub name: String,
    pub chips_amount: u32,
    pub cards: Vec<Card>,
    pub current_bet: u32,
    pub is_folded: bool,
    pub is_all_in: bool,
    pub is_dealer: bool,
}

impl Player {
    pub fn new(name: String, initial_chips: u32) -> Self {
        Self {
            id: PlayerId::new(),
            name,
            hand: Hand::new(5), // デフォルトの手札サイズは5
            chips: Chips::new(initial_chips),
            current_bet: 0,
            is_folded: false,
            is_all_in: false,
            is_dealer: false,
        }
    }
    
    /// デシリアライズ用のファクトリメソッド
    pub fn from_serialized(data: PlayerSerializedData) -> Result<Self, DomainError> {
        let mut hand = Hand::new(5);
        for card in data.cards {
            hand.add_card(card).map_err(|e| DomainError::InvalidCard(e.to_string()))?;
        }
        
        Ok(Self {
            id: data.id,
            name: data.name,
            hand,
            chips: Chips::new(data.chips_amount),
            current_bet: data.current_bet,
            is_folded: data.is_folded,
            is_all_in: data.is_all_in,
            is_dealer: data.is_dealer,
        })
    }
    
    pub fn id(&self) -> &PlayerId {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn hand(&self) -> &Hand {
        &self.hand
    }
    
    pub fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }
    
    pub fn chips(&self) -> u32 {
        self.chips.amount()
    }
    
    pub fn add_chips(&mut self, amount: u32) {
        self.chips.add(amount);
    }
    
    pub fn current_bet(&self) -> u32 {
        self.current_bet
    }
    
    pub fn is_folded(&self) -> bool {
        self.is_folded
    }
    
    pub fn is_all_in(&self) -> bool {
        self.is_all_in
    }
    
    pub fn is_dealer(&self) -> bool {
        self.is_dealer
    }
    
    pub fn set_dealer(&mut self, is_dealer: bool) {
        self.is_dealer = is_dealer;
    }
    
    pub fn fold(&mut self) {
        self.is_folded = true;
    }
    
    pub fn place_bet(&mut self, amount: u32) -> Result<u32, DomainError> {
        if amount == 0 {
            return Ok(0);
        }
        
        // チップが足りない場合はオールイン
        let bet_amount = if amount > self.chips.amount() {
            let available = self.chips.amount();
            self.chips = Chips::new(0);
            self.is_all_in = true;
            available
        } else {
            self.chips.subtract(amount).map_err(|e| DomainError::from(e))?;
            if self.chips.is_zero() {
                self.is_all_in = true;
            }
            amount
        };
        
        self.current_bet += bet_amount;
        Ok(bet_amount)
    }
    
    pub fn reset_bet(&mut self) {
        self.current_bet = 0;
    }
    
    pub fn reset_for_new_round(&mut self) {
        self.hand.clear();
        self.is_folded = false;
        self.current_bet = 0;
    }
    
    pub fn reset_for_new_game(&mut self) {
        self.reset_for_new_round();
        self.is_all_in = false;
        self.is_dealer = false;
    }
    
    pub fn can_afford(&self, amount: u32) -> bool {
        self.chips.amount() >= amount
    }

    /// 手札にカードを追加
    pub fn add_card_to_hand(&mut self, card: Card) -> Result<(), DomainError> {
        self.hand.add_card(card).map_err(|e| DomainError::InvalidCard(e.to_string()))
    }

    /// 手札をリセット
    pub fn reset_hand(&mut self, max_size: usize) -> Result<(), DomainError> {
        self.hand = Hand::new(max_size);
        Ok(())
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}チップ){}{}",
            self.name,
            self.chips.amount(),
            if self.is_dealer { " [ディーラー]" } else { "" },
            if self.is_all_in { " [オールイン]" } else { "" }
        )
    }
} 