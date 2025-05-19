use crate::domain::model::error::{DomainError, PlayerError};
use crate::domain::model::player::{Player, PlayerId};
use crate::domain::model::game::GamePhase;
use crate::domain::model::card::Card;
use serde::{Serialize, Deserialize};

/// プレイヤー管理を担当するクラス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerManager {
    players: Vec<Player>,
    max_players: usize,
}

impl PlayerManager {
    /// 新しいプレイヤーマネージャーを作成
    pub fn new(max_players: usize) -> Self {
        Self {
            players: Vec::new(),
            max_players,
        }
    }

    // --- ゲッター ---
    pub fn players(&self) -> &[Player] {
        &self.players
    }
    
    pub fn players_mut(&mut self) -> &mut [Player] {
        &mut self.players
    }
    
    pub fn player_at(&self, index: usize) -> Result<&Player, DomainError> {
        self.players.get(index).ok_or_else(|| DomainError::InvalidGameOperation(
            format!("無効なプレイヤーインデックス: {}", index)
        ))
    }
    
    pub fn player_at_mut(&mut self, index: usize) -> Result<&mut Player, DomainError> {
        self.players.get_mut(index).ok_or_else(|| DomainError::InvalidGameOperation(
            format!("無効なプレイヤーインデックス: {}", index)
        ))
    }
    
    pub fn find_player_by_id(&self, id: &PlayerId) -> Option<(usize, &Player)> {
        self.players.iter().enumerate()
            .find(|(_, player)| player.id() == id)
    }
    
    pub fn find_player_by_id_mut(&mut self, id: &PlayerId) -> Option<(usize, &mut Player)> {
        self.players.iter_mut().enumerate()
            .find(|(_, player)| player.id() == id)
    }
    
    pub fn count(&self) -> usize {
        self.players.len()
    }
    
    pub fn active_player_count(&self) -> usize {
        self.players.iter().filter(|p| !p.is_folded() && !p.is_all_in()).count()
    }
    
    // --- プレイヤー管理 ---
    
    /// プレイヤーを追加
    pub fn add_player(&mut self, player: Player, game_phase: GamePhase) -> Result<(), DomainError> {
        if game_phase != GamePhase::NotStarted {
            return Err(DomainError::InvalidPhase {
                expected: GamePhase::NotStarted,
                actual: game_phase,
            });
        }
        
        if self.players.len() >= self.max_players {
            return Err(DomainError::InvalidGameOperation(
                format!("プレイヤーの最大数({})に達しています", self.max_players)
            ));
        }
        
        // 既に同じIDのプレイヤーがいないか確認
        if self.players.iter().any(|p| p.id() == player.id()) {
            return Err(DomainError::InvalidGameOperation(
                format!("プレイヤーID {} はすでに使用されています", player.id())
            ));
        }
        
        self.players.push(player);
        Ok(())
    }
    
    /// カードを全プレイヤーに配る
    pub fn deal_cards_to_players(&mut self, cards_per_player: usize, cards: Vec<Vec<Card>>) -> Result<(), DomainError> {
        if cards.len() != self.players.len() {
            return Err(DomainError::InvalidGameOperation(
                format!("配布するカードセット数({})とプレイヤー数({})が一致しません", 
                cards.len(), self.players.len())
            ));
        }
        
        for (i, player_cards) in cards.into_iter().enumerate() {
            let player = &mut self.players[i];
            player.reset_hand(cards_per_player)?;
            
            for card in player_cards {
                player.add_card_to_hand(card)?;
            }
        }
        
        Ok(())
    }
    
    /// ディーラーを設定
    pub fn set_dealer(&mut self, index: usize) -> Result<(), DomainError> {
        if index >= self.players.len() {
            return Err(DomainError::InvalidGameOperation(
                format!("無効なディーラーインデックス: {}", index)
            ));
        }
        
        // 全プレイヤーのディーラーフラグをリセット
        for player in &mut self.players {
            player.set_dealer(false);
        }
        
        // 指定したプレイヤーをディーラーに設定
        self.players[index].set_dealer(true);
        
        Ok(())
    }
    
    /// 次のアクティブなプレイヤーを探す
    pub fn find_next_active_player(&self, from_index: usize) -> usize {
        if self.players.is_empty() {
            return 0;
        }
        
        let mut index = (from_index + 1) % self.players.len();
        let start_index = index;
        
        loop {
            if !self.players[index].is_folded() && !self.players[index].is_all_in() {
                return index;
            }
            
            index = (index + 1) % self.players.len();
            if index == start_index {
                // 一周してきた場合は元のインデックスを返す
                return from_index;
            }
        }
    }
    
    /// ブラインドを投入
    pub fn post_blinds(&mut self, dealer_index: usize, small_blind: u32, big_blind: u32) -> Result<u32, DomainError> {
        if self.players.len() < 2 {
            return Err(DomainError::InvalidGameOperation(
                "ブラインドを投入するには最低2人のプレイヤーが必要です".into()
            ));
        }
        
        let small_blind_index = self.find_next_active_player(dealer_index);
        let small_blind_amount = self.players[small_blind_index].place_bet(small_blind)?;
        
        let big_blind_index = self.find_next_active_player(small_blind_index);
        let big_blind_amount = self.players[big_blind_index].place_bet(big_blind)?;
        
        let total_blind = small_blind_amount + big_blind_amount;
        
        Ok(total_blind)
    }
    
    /// 全プレイヤーのベットをリセット
    pub fn reset_all_bets(&mut self) {
        for player in &mut self.players {
            player.reset_bet();
        }
    }
    
    /// 新しいラウンド用にプレイヤー状態をリセット
    pub fn reset_for_new_round(&mut self) {
        for player in &mut self.players {
            player.reset_for_new_round();
        }
    }
    
    /// 新しいゲーム用にプレイヤー状態をリセット
    pub fn reset_for_new_game(&mut self) {
        for player in &mut self.players {
            player.reset_for_new_game();
        }
    }
    
    /// 勝者にポットを分配
    pub fn distribute_pot_to_winners(&mut self, winners: &[(usize, u32)]) -> Result<(), DomainError> {
        for &(winner_index, amount) in winners {
            if winner_index >= self.players.len() {
                return Err(DomainError::InvalidGameOperation(
                    format!("無効な勝者インデックス: {}", winner_index)
                ));
            }
            
            self.players[winner_index].add_chips(amount);
        }
        
        Ok(())
    }
} 