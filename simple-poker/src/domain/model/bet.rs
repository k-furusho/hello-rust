use std::fmt;
use serde::{Serialize, Deserialize};
use super::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetAction {
    Fold,
    Check,
    Call,
    Raise,
    AllIn,
}

impl fmt::Display for BetAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action_str = match self {
            BetAction::Fold => "フォールド",
            BetAction::Check => "チェック",
            BetAction::Call => "コール",
            BetAction::Raise => "レイズ",
            BetAction::AllIn => "オールイン",
        };
        write!(f, "{}", action_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BetAmount(u32);

impl BetAmount {
    pub fn new(amount: u32) -> Self {
        Self(amount)
    }
    
    pub fn value(&self) -> u32 {
        self.0
    }
    
    pub fn zero() -> Self {
        Self(0)
    }
    
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    
    pub fn add(&self, other: BetAmount) -> Self {
        Self(self.0 + other.0)
    }
    
    pub fn subtract(&self, other: BetAmount) -> Result<Self, DomainError> {
        if other.0 > self.0 {
            return Err(DomainError::InvalidBet(format!("引き出そうとしている額({})が利用可能な額({})を超えています", other.0, self.0)));
        }
        Ok(Self(self.0 - other.0))
    }
}

impl From<u32> for BetAmount {
    fn from(amount: u32) -> Self {
        Self(amount)
    }
}

impl fmt::Display for BetAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} チップ", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chips(u32);

impl Chips {
    pub fn new(amount: u32) -> Self {
        Self(amount)
    }
    
    pub fn amount(&self) -> u32 {
        self.0
    }
    
    pub fn add(&mut self, amount: u32) {
        self.0 += amount;
    }
    
    pub fn add_bet_amount(&mut self, amount: BetAmount) {
        self.0 += amount.value();
    }
    
    pub fn subtract(&mut self, amount: u32) -> Result<(), DomainError> {
        if amount > self.0 {
            return Err(DomainError::InvalidBet("チップが足りません".into()));
        }
        self.0 -= amount;
        Ok(())
    }
    
    pub fn subtract_bet_amount(&mut self, amount: BetAmount) -> Result<(), DomainError> {
        self.subtract(amount.value())
    }
    
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Chips {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} チップ", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Pot {
    chips: Chips,
    side_pots: Vec<(Chips, Vec<String>)>, // (チップ額, 対象プレイヤーID)
}

impl Pot {
    pub fn new() -> Self {
        Self {
            chips: Chips::new(0),
            side_pots: Vec::new(),
        }
    }
    
    pub fn add(&mut self, amount: u32) {
        self.chips.add(amount);
    }
    
    pub fn add_bet(&mut self, amount: BetAmount) {
        self.chips.add_bet_amount(amount);
    }
    
    pub fn total(&self) -> u32 {
        self.chips.amount() + self.side_pots.iter().map(|(chips, _)| chips.amount()).sum::<u32>()
    }
    
    pub fn main_pot(&self) -> u32 {
        self.chips.amount()
    }
    
    pub fn create_side_pot(&mut self, amount: u32, player_ids: Vec<String>) {
        self.side_pots.push((Chips::new(amount), player_ids));
    }
    
    pub fn side_pots(&self) -> &[(Chips, Vec<String>)] {
        &self.side_pots
    }
    
    pub fn clear(&mut self) {
        self.chips = Chips::new(0);
        self.side_pots.clear();
    }
} 