use std::collections::HashMap;
use crate::domain::model::card::Card;

pub trait HandSpecification {
    /// この手札が条件を満たすかどうかを判定する
    fn is_satisfied_by(&self, cards: &[Card]) -> bool;
    /// 仕様の名前を返す
    fn name(&self) -> &'static str;
}

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

pub struct StraightSpecification;

impl HandSpecification for StraightSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() != 5 {
            return false;
        }
        
        // カードをランク順にソート
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
        ranks.sort();
        
        // A-2-3-4-5のホイールストレート
        if ranks == [1, 2, 3, 4, 5] {
            return true;
        }
        
        // 10-J-Q-K-Aのロイヤルストレート
        if ranks == [1, 10, 11, 12, 13] {
            return true;
        }
        
        // 重複がないか確認
        for i in 0..ranks.len() - 1 {
            if ranks[i] == ranks[i + 1] {
                return false;
            }
        }
        
        // 連続しているか確認
        for i in 0..ranks.len() - 1 {
            if ranks[i + 1] != ranks[i] + 1 {
                return false;
            }
        }
        
        true
    }
    
    fn name(&self) -> &'static str {
        "ストレート"
    }
}

pub struct RoyalStraightFlushSpecification;

impl HandSpecification for RoyalStraightFlushSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() != 5 {
            return false;
        }
        
        // 同じスートか確認
        let first_suit = cards[0].suit();
        if !cards.iter().all(|card| card.suit() == first_suit) {
            return false;
        }
        
        // 10, J, Q, K, A のランクを持っているか確認
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
        ranks.sort();
        
        ranks == [1, 10, 11, 12, 13]
    }
    
    fn name(&self) -> &'static str {
        "ロイヤルストレートフラッシュ"
    }
}

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

pub struct FullHouseSpecification;

impl HandSpecification for FullHouseSpecification {
    fn is_satisfied_by(&self, cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        let rank_counts = count_ranks(cards);
        let has_three = rank_counts.values().any(|&count| count >= 3);
        let pair_count = rank_counts.values().filter(|&&count| count >= 2).count();
        
        has_three && pair_count >= 2
    }
    
    fn name(&self) -> &'static str {
        "フルハウス"
    }
}

fn count_ranks(cards: &[Card]) -> HashMap<u8, u8> {
    let mut rank_counts = HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank()).or_insert(0) += 1;
    }
    rank_counts
}

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