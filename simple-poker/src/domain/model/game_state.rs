use crate::domain::model::error::DomainError;
use crate::domain::model::game::{GameId, GamePhase, BettingRound, GameVariant};
use serde::{Serialize, Deserialize};

/// ゲーム状態を管理するクラス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    id: GameId,
    current_phase: GamePhase,
    current_round: Option<BettingRound>,
    current_player_index: usize,
    dealer_index: usize,
    small_blind: u32,
    big_blind: u32,
    current_bet: u32,
    variant: GameVariant,
    max_players: usize,
    active_player_count: usize,
}

impl GameState {
    /// 新しいゲーム状態を作成
    pub fn new(
        variant: GameVariant, 
        small_blind: u32, 
        big_blind: u32,
        max_players: usize
    ) -> Result<Self, DomainError> {
        if small_blind >= big_blind {
            return Err(DomainError::InvalidGameOperation(
                format!("スモールブラインド({})はビッグブラインド({})より小さくなければなりません", 
                small_blind, big_blind)
            ));
        }

        Ok(Self {
            id: GameId::new(),
            current_phase: GamePhase::NotStarted,
            current_round: None,
            current_player_index: 0,
            dealer_index: 0,
            small_blind,
            big_blind,
            current_bet: 0,
            variant,
            max_players,
            active_player_count: 0,
        })
    }

    /// 既存のIDからゲーム状態を作成
    pub fn with_id(
        id: GameId,
        variant: GameVariant, 
        small_blind: u32, 
        big_blind: u32,
        max_players: usize
    ) -> Result<Self, DomainError> {
        let mut state = Self::new(variant, small_blind, big_blind, max_players)?;
        state.id = id;
        Ok(state)
    }

    // --- ゲッター ---
    pub fn id(&self) -> &GameId {
        &self.id
    }
    
    pub fn current_phase(&self) -> GamePhase {
        self.current_phase
    }
    
    pub fn current_round(&self) -> Option<BettingRound> {
        self.current_round
    }
    
    pub fn current_player_index(&self) -> usize {
        self.current_player_index
    }
    
    pub fn dealer_index(&self) -> usize {
        self.dealer_index
    }
    
    pub fn small_blind(&self) -> u32 {
        self.small_blind
    }
    
    pub fn big_blind(&self) -> u32 {
        self.big_blind
    }
    
    pub fn current_bet(&self) -> u32 {
        self.current_bet
    }
    
    pub fn variant(&self) -> GameVariant {
        self.variant
    }

    pub fn max_players(&self) -> usize {
        self.max_players
    }

    // --- セッター ---
    pub fn set_current_phase(&mut self, phase: GamePhase) {
        self.current_phase = phase;
    }
    
    pub fn set_current_round(&mut self, round: Option<BettingRound>) {
        self.current_round = round;
    }
    
    pub fn set_current_player_index(&mut self, index: usize) {
        self.current_player_index = index;
    }
    
    pub fn set_dealer_index(&mut self, index: usize) {
        self.dealer_index = index;
    }
    
    pub fn set_current_bet(&mut self, amount: u32) {
        self.current_bet = amount;
    }

    pub fn set_active_player_count(&mut self, count: usize) {
        self.active_player_count = count;
    }

    // --- 状態管理メソッド ---
    
    /// 次のプレイヤーに進む
    pub fn advance_to_next_player(&mut self, next_index: usize) {
        self.current_player_index = next_index;
    }
    
    /// 次のラウンドに進む
    pub fn advance_to_next_round(&mut self, next_round: Option<BettingRound>) -> Result<(), DomainError> {
        match next_round {
            Some(round) => {
                self.current_round = Some(round);
                self.current_bet = 0;
                Ok(())
            },
            None => {
                self.set_current_phase(GamePhase::Showdown);
                Ok(())
            }
        }
    }
    
    /// ゲームの完了
    pub fn complete_game(&mut self) {
        self.current_phase = GamePhase::Complete;
    }
    
    /// 新しいラウンド用にステートをリセット
    pub fn reset_for_new_round(&mut self) {
        self.current_bet = 0;
    }
    
    /// 新しいゲーム用にステートをリセット
    pub fn reset_for_new_game(&mut self) {
        self.current_phase = GamePhase::NotStarted;
        self.current_round = None;
        self.current_bet = 0;
        self.current_player_index = 0;
        
        // ディーラーを次のプレイヤーに移動
        self.dealer_index = (self.dealer_index + 1) % self.active_player_count.max(1);
    }
    
    /// フェーズの検証
    pub fn validate_phase(&self, expected: GamePhase) -> Result<(), DomainError> {
        if self.current_phase != expected {
            return Err(DomainError::InvalidPhase {
                expected,
                actual: self.current_phase,
            });
        }
        Ok(())
    }
    
    /// インデックスの検証
    pub fn validate_index(&self, index: usize, max: usize, name: &str) -> Result<(), DomainError> {
        if index >= max {
            return Err(DomainError::InvalidGameOperation(
                format!("無効な{}インデックスです: {}", name, index)
            ));
        }
        Ok(())
    }
} 