use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    
    pub fn subtract(&mut self, amount: u32) -> Result<(), &'static str> {
        if amount > self.0 {
            return Err("チップが足りません");
        }
        self.0 -= amount;
        Ok(())
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