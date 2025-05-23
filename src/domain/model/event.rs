use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::domain::model::game::{GameId, GameVariant, BettingRound};
use crate::domain::model::player::PlayerId;
use crate::domain::model::card::Card;
use super::bet::BetAction;

pub trait DomainEvent: std::fmt::Debug {
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn aggregate_id(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    GameCreated {
        game_id: GameId,
        variant: GameVariant,
        small_blind: u32,
        big_blind: u32,
        time: DateTime<Utc>,
    },
    
    GameStarted {
        game_id: GameId,
        player_count: usize,
        time: DateTime<Utc>,
    },
    
    PlayerAdded {
        game_id: GameId,
        player_id: PlayerId,
        player_name: String,
        initial_chips: u32,
        time: DateTime<Utc>,
    },
    
    CardsDealt {
        game_id: GameId,
        player_id: PlayerId,
        time: DateTime<Utc>,
    },
    
    BettingRoundStarted {
        game_id: GameId,
        round: BettingRound,
        time: DateTime<Utc>,
    },
    
    PlayerAction {
        game_id: GameId,
        player_id: PlayerId,
        action: BetAction,
        amount: Option<u32>,
        time: DateTime<Utc>,
    },
    
    CardsExchanged {
        game_id: GameId,
        player_id: PlayerId,
        count: usize,
        time: DateTime<Utc>,
    },
    
    CommunityCardsDealt {
        game_id: GameId,
        cards: Vec<Card>,
        time: DateTime<Utc>,
    },
    
    GameEnded {
        game_id: GameId,
        winner_ids: Vec<PlayerId>,
        pot_amount: u32,
        time: DateTime<Utc>,
    },
}

impl DomainEvent for GameEvent {
    fn event_type(&self) -> &'static str {
        match self {
            GameEvent::GameCreated { .. } => "game_created",
            GameEvent::GameStarted { .. } => "game_started",
            GameEvent::PlayerAdded { .. } => "player_added",
            GameEvent::CardsDealt { .. } => "cards_dealt",
            GameEvent::BettingRoundStarted { .. } => "betting_round_started",
            GameEvent::PlayerAction { .. } => "player_action",
            GameEvent::CardsExchanged { .. } => "cards_exchanged",
            GameEvent::CommunityCardsDealt { .. } => "community_cards_dealt",
            GameEvent::GameEnded { .. } => "game_ended",
        }
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            GameEvent::GameCreated { time, .. } => *time,
            GameEvent::GameStarted { time, .. } => *time,
            GameEvent::PlayerAdded { time, .. } => *time,
            GameEvent::CardsDealt { time, .. } => *time,
            GameEvent::BettingRoundStarted { time, .. } => *time,
            GameEvent::PlayerAction { time, .. } => *time,
            GameEvent::CardsExchanged { time, .. } => *time,
            GameEvent::CommunityCardsDealt { time, .. } => *time,
            GameEvent::GameEnded { time, .. } => *time,
        }
    }
    
    fn aggregate_id(&self) -> String {
        match self {
            GameEvent::GameCreated { game_id, .. } => game_id.value().to_string(),
            GameEvent::GameStarted { game_id, .. } => game_id.value().to_string(),
            GameEvent::PlayerAdded { game_id, .. } => game_id.value().to_string(),
            GameEvent::CardsDealt { game_id, .. } => game_id.value().to_string(),
            GameEvent::BettingRoundStarted { game_id, .. } => game_id.value().to_string(),
            GameEvent::PlayerAction { game_id, .. } => game_id.value().to_string(),
            GameEvent::CardsExchanged { game_id, .. } => game_id.value().to_string(),
            GameEvent::CommunityCardsDealt { game_id, .. } => game_id.value().to_string(),
            GameEvent::GameEnded { game_id, .. } => game_id.value().to_string(),
        }
    }
}

pub trait EventPublisher {
    fn publish(&self, event: GameEvent) -> Result<(), super::error::DomainError>;
}

pub trait EventSubscriber {
    fn subscribe(&mut self, callback: Box<dyn Fn(&GameEvent) + Send + 'static>);
} 