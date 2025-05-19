use crate::domain::model::error::{DomainError, DeckError};
use crate::domain::model::bet::Pot;
use crate::domain::model::deck::Deck;
use crate::domain::model::card::Card;
use crate::domain::model::game::BettingRound;
use crate::domain::model::game::GameVariant;
use serde::{Serialize, Deserialize};

/// ゲームテーブルの状態を管理するクラス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTable {
    deck: Deck,
    pot: Pot,
    community_cards: Vec<Card>,
}

impl GameTable {
    /// 新しいゲームテーブルを作成
    pub fn new() -> Result<Self, DomainError> {
        Ok(Self {
            deck: Deck::new()?,
            pot: Pot::new(),
            community_cards: Vec::new(),
        })
    }

    // --- ゲッター ---
    pub fn pot(&self) -> &Pot {
        &self.pot
    }
    
    pub fn pot_mut(&mut self) -> &mut Pot {
        &mut self.pot
    }
    
    pub fn community_cards(&self) -> &[Card] {
        &self.community_cards
    }
    
    // --- デッキ操作 ---
    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }
    
    pub fn draw_card(&mut self) -> Result<Card, DomainError> {
        self.deck.draw().ok_or(DomainError::DeckError(DeckError::EmptyDeck))
    }
    
    pub fn draw_cards(&mut self, count: usize) -> Result<Vec<Card>, DomainError> {
        let mut cards = Vec::with_capacity(count);
        for _ in 0..count {
            cards.push(self.draw_card()?);
        }
        Ok(cards)
    }
    
    pub fn return_card_to_deck(&mut self, card: Card) {
        self.deck.add_card(card);
    }
    
    // --- コミュニティカード操作 ---
    
    /// フロップカードを配る (3枚)
    pub fn deal_flop(&mut self) -> Result<(), DomainError> {
        for _ in 0..3 {
            let card = self.draw_card()?;
            self.community_cards.push(card);
        }
        Ok(())
    }
    
    /// ターンかリバーを配る (1枚)
    pub fn deal_turn_or_river(&mut self) -> Result<(), DomainError> {
        let card = self.draw_card()?;
        self.community_cards.push(card);
        Ok(())
    }
    
    /// ラウンドに応じたコミュニティカードを配る
    pub fn deal_community_cards_for_round(&mut self, round: BettingRound, variant: GameVariant) -> Result<(), DomainError> {
        match (variant, round) {
            (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::Flop) => {
                self.deal_flop()?;
            },
            (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::Turn | BettingRound::River) => {
                self.deal_turn_or_river()?;
            },
            _ => {} // 他の場合は何もしない
        }
        
        Ok(())
    }
    
    /// カードを交換する
    pub fn exchange_card(&mut self, old_card: Card) -> Result<Card, DomainError> {
        let new_card = self.draw_card()?;
        self.return_card_to_deck(old_card);
        Ok(new_card)
    }
    
    // --- ポット操作 ---
    pub fn add_to_pot(&mut self, amount: u32) {
        self.pot.add(amount);
    }
    
    pub fn pot_total(&self) -> u32 {
        self.pot.total()
    }
    
    pub fn clear_pot(&mut self) {
        self.pot.clear();
    }
    
    // --- リセット ---
    pub fn reset_for_new_hand(&mut self) -> Result<(), DomainError> {
        self.deck = Deck::new()?;
        self.pot.clear();
        self.community_cards.clear();
        
        Ok(())
    }
} 