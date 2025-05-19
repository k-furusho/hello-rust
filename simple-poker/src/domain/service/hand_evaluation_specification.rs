use std::collections::HashMap;
use crate::domain::model::card::{Card, Suit};
use crate::domain::model::hand::Hand;

/// 手札の評価条件を表す仕様インターフェース
pub trait HandSpecification {
    /// この手札が条件を満たすかどうかを判定する
    fn is_satisfied_by(&self, cards: &[Card]) -> bool;
    /// 仕様の名前を返す
    fn name(&self) -> &'static str;
}

/// フラッシュ（同じスートのカード5枚）の仕様
pub struct FlushSpecification;

impl HandSpecification for FlushSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        
        let first_suit = cards[0].suit();
        cards.iter().all(|card| card.suit() == first_suit)
    }
    
    fn name(&self) -> &'static str {
        "フラッシュ"
    }
}

/// ストレート（連続した数字のカード5枚）の仕様
pub struct StraightSpecification;

impl HandSpecification for StraightSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
        ranks.sort();
        
        // 重複を除去
        ranks.dedup();
        
        if ranks.len() < 5 {
            return false;
        }
        
        // A-2-3-4-5のストレート
        if ranks.contains(&1) && ranks.contains(&2) && ranks.contains(&3) 
           && ranks.contains(&4) && ranks.contains(&5) {
            return true;
        }
        
        // 10-J-Q-K-Aのストレート
        if ranks.contains(&1) && ranks.contains(&10) && ranks.contains(&11) 
           && ranks.contains(&12) && ranks.contains(&13) {
            return true;
        }
        
        // 通常の5枚連続をチェック
        for window in ranks.windows(5) {
            if window[4] - window[0] == 4 {
                return true;
            }
        }
        
        false
    }
    
    fn name(&self) -> &'static str {
        "ストレート"
    }
}

/// ロイヤルストレートフラッシュの仕様
pub struct RoyalStraightFlushSpecification;

impl HandSpecification for RoyalStraightFlushSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        
        // 同じスートであることを確認
        let flush_spec = FlushSpecification;
        if !flush_spec.is_satisfied_by(cards) {
            return false;
        }
        
        // 必要なランクがあるかチェック
        let mut has_ace = false;
        let mut has_ten = false;
        let mut has_jack = false;
        let mut has_queen = false;
        let mut has_king = false;
        
        for card in cards {
            match card.rank() {
                1 => has_ace = true,
                10 => has_ten = true,
                11 => has_jack = true,
                12 => has_queen = true,
                13 => has_king = true,
                _ => {}
            }
        }
        
        has_ace && has_ten && has_jack && has_queen && has_king
    }
    
    fn name(&self) -> &'static str {
        "ロイヤルストレートフラッシュ"
    }
}

/// フォーカード（同じランクのカード4枚）の仕様
pub struct FourOfAKindSpecification;

impl HandSpecification for FourOfAKindSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() < 4 {
            return false;
        }
        
        let rank_counts = count_ranks(cards);
        rank_counts.values().any(|&count| count >= 4)
    }
    
    fn name(&self) -> &'static str {
        "フォーカード"
    }
}

/// フルハウス（スリーカード + ワンペア）の仕様
pub struct FullHouseSpecification;

impl HandSpecification for FullHouseSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        
        let rank_counts = count_ranks(cards);
        let has_three = rank_counts.values().any(|&count| count >= 3);
        let pair_count = rank_counts.values().filter(|&&count| count >= 2).count();
        
        has_three && pair_count >= 2 // スリーカードが有り、かつペアが2つ以上ある
    }
    
    fn name(&self) -> &'static str {
        "フルハウス"
    }
}

/// ランクの出現回数をカウントするヘルパー関数
fn count_ranks(cards: &[Card]) -> HashMap<u8, u8> {
    let mut rank_counts = HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank()).or_insert(0) += 1;
    }
    rank_counts
}

/// 複合仕様 - 論理AND
pub struct AndSpecification<T, U> {
    spec1: T,
    spec2: U,
    name: &'static str,
}

impl<T, U> AndSpecification<T, U> {
    pub fn new(spec1: T, spec2: U, name: &'static str) -> Self {
        Self { spec1, spec2, name }
    }
}

impl<T: HandSpecification, U: HandSpecification> HandSpecification for AndSpecification<T, U> {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        self.spec1.is_satisfied_by(cards) && self.spec2.is_satisfied_by(cards)
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
}

/// 複合仕様 - 論理OR
pub struct OrSpecification<T, U> {
    spec1: T,
    spec2: U,
    name: &'static str,
}

impl<T, U> OrSpecification<T, U> {
    pub fn new(spec1: T, spec2: U, name: &'static str) -> Self {
        Self { spec1, spec2, name }
    }
}

impl<T: HandSpecification, U: HandSpecification> HandSpecification for OrSpecification<T, U> {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        self.spec1.is_satisfied_by(cards) || self.spec2.is_satisfied_by(cards)
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
}

/// 複合仕様 - 論理NOT
pub struct NotSpecification<T> {
    spec: T,
    name: &'static str,
}

impl<T> NotSpecification<T> {
    pub fn new(spec: T, name: &'static str) -> Self {
        Self { spec, name }
    }
}

impl<T: HandSpecification> HandSpecification for NotSpecification<T> {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        !self.spec.is_satisfied_by(cards)
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
} 