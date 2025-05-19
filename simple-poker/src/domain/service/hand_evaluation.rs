use std::collections::HashMap;

use crate::domain::model::card::Card;
use crate::domain::model::game::GameVariant;

/// 役の強さを表す値のリスト（高いカードからのランク値）
type HandValues = Vec<u8>;
/// カード枚数のマッピング（ランク → 枚数）
type RankCountMap = HashMap<u8, u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    RoyalStraightFlush = 9,
    StraightFlush = 8,
    FourOfAKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl std::fmt::Display for HandRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rank_str = match self {
            HandRank::RoyalStraightFlush => "ロイヤルストレートフラッシュ",
            HandRank::StraightFlush => "ストレートフラッシュ",
            HandRank::FourOfAKind => "フォーカード",
            HandRank::FullHouse => "フルハウス",
            HandRank::Flush => "フラッシュ",
            HandRank::Straight => "ストレート",
            HandRank::ThreeOfAKind => "スリーカード",
            HandRank::TwoPair => "ツーペア",
            HandRank::OnePair => "ワンペア",
            HandRank::HighCard => "ハイカード",
        };
        write!(f, "{}", rank_str)
    }
}

// 手の強さを表す構造体（役とタイブレーク用の情報）
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandStrength {
    rank: HandRank,
    values: HandValues, // ここで型エイリアスを使用
}

impl HandStrength {
    pub fn new(rank: HandRank, values: HandValues) -> Self { // ここでも使用
        Self { rank, values }
    }
    
    pub fn rank(&self) -> HandRank {
        self.rank
    }
    
    pub fn values(&self) -> &HandValues { // ここでも使用
        &self.values
    }
}

pub struct HandEvaluationService;

impl HandEvaluationService {
    // 手札の役を評価する
    pub fn evaluate_hand(cards: &[Card]) -> HandStrength {
        if cards.is_empty() {
            return HandStrength::new(HandRank::HighCard, vec![]);
        }
        
        let is_flush = Self::is_flush(cards);
        let is_straight = Self::is_straight(cards);
        let is_royal = Self::is_royal(cards);
        let rank_counts = Self::count_ranks(cards);
        
        match (is_flush, is_straight, is_royal) {
            (true, true, true) => {
                HandStrength::new(HandRank::RoyalStraightFlush, vec![14]) // Aを14として扱う
            },
            (true, true, false) => {
                let high_card = Self::get_high_card_value(cards);
                HandStrength::new(HandRank::StraightFlush, vec![high_card])
            },
            (true, false, _) => {
                let values = Self::get_values_for_flush(cards);
                HandStrength::new(HandRank::Flush, values)
            },
            (false, true, _) => {
                let high_card = Self::get_high_card_value(cards);
                HandStrength::new(HandRank::Straight, vec![high_card])
            },
            _ => Self::evaluate_by_counts(rank_counts, cards),
        }
    }
    
    // ホールデムやオマハで最適な5枚の組み合わせを見つける
    pub fn find_best_hand(hand: &[Card], community: &[Card], variant: GameVariant) -> HandStrength {
        match variant {
            GameVariant::FiveCardDraw => Self::evaluate_hand(hand),
            GameVariant::TexasHoldem => Self::best_holdem_hand(hand, community),
            GameVariant::Omaha => Self::best_omaha_hand(hand, community),
        }
    }
    
    // テキサスホールデム用の最適な手札評価
    fn best_holdem_hand(hole_cards: &[Card], community_cards: &[Card]) -> HandStrength {
        let mut all_cards = Vec::with_capacity(hole_cards.len() + community_cards.len());
        all_cards.extend_from_slice(hole_cards);
        all_cards.extend_from_slice(community_cards);
        
        // 全部の組み合わせから最強のものを探す
        Self::find_best_five_card_hand(&all_cards)
    }
    
    // オマハ用の最適な手札評価（2枚のホールカードと3枚のコミュニティカードを使用）
    fn best_omaha_hand(hole_cards: &[Card], community_cards: &[Card]) -> HandStrength {
        if hole_cards.len() < 4 || community_cards.len() < 3 {
            return HandStrength::new(HandRank::HighCard, vec![]);
        }
        
        let mut best_hand = HandStrength::new(HandRank::HighCard, vec![]);
        
        // ホールカードから2枚を選ぶ組み合わせ
        for i in 0..hole_cards.len() {
            for j in i+1..hole_cards.len() {
                // コミュニティカードから3枚を選ぶ組み合わせ
                for k in 0..community_cards.len() {
                    for l in k+1..community_cards.len() {
                        for m in l+1..community_cards.len() {
                            let hand = [
                                hole_cards[i], hole_cards[j],
                                community_cards[k], community_cards[l], community_cards[m],
                            ];
                            
                            let strength = Self::evaluate_hand(&hand);
                            if strength > best_hand {
                                best_hand = strength;
                            }
                        }
                    }
                }
            }
        }
        
        best_hand
    }
    
    // 7枚のカードから最適な5枚を見つける
    fn find_best_five_card_hand(cards: &[Card]) -> HandStrength {
        if cards.len() <= 5 {
            return Self::evaluate_hand(cards);
        }
        
        let mut best_hand = HandStrength::new(HandRank::HighCard, vec![]);
        
        // 5枚のカードを選ぶすべての組み合わせを評価
        let mut indices = [0, 1, 2, 3, 4];
        let n = cards.len();
        
        loop {
            let hand = [
                cards[indices[0]], cards[indices[1]], cards[indices[2]],
                cards[indices[3]], cards[indices[4]],
            ];
            
            let strength = Self::evaluate_hand(&hand);
            if strength > best_hand {
                best_hand = strength;
            }
            
            // 次の組み合わせに進む
            let mut i = 4;
            loop {
                if indices[i] == n - 5 + i {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                } else {
                    break;
                }
            }
            if i == 0 && indices[i] == n - 5 + i {
                break; // すべての組み合わせを試した
            }
            indices[i] += 1;
            for j in i+1..5 {
                indices[j] = indices[j-1] + 1;
            }
        }
        
        best_hand
    }
    
    // フラッシュかどうかを判定
    fn is_flush(cards: &[Card]) -> bool {
        if cards.is_empty() {
            return false;
        }
        
        let first_suit = cards[0].suit();
        cards.iter().all(|card| card.suit() == first_suit)
    }
    
    // ストレートかどうかを判定
    fn is_straight(cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
        ranks.sort();
        ranks.dedup(); // 重複を削除
        
        if ranks.len() < 5 {
            return false;
        }
        
        // 通常のストレートチェック
        for window in ranks.windows(5) {
            if window[4] - window[0] == 4 {
                return true;
            }
        }
        
        // A-5-4-3-2のケース（Aを1として扱う場合）
        if ranks.contains(&1) && ranks.contains(&2) && ranks.contains(&3) && ranks.contains(&4) && ranks.contains(&5) {
            return true;
        }
        
        false
    }
    
    // ロイヤルストレートフラッシュかどうかを判定
    fn is_royal(cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        
        let mut has_ace = false;
        let mut has_king = false;
        let mut has_queen = false;
        let mut has_jack = false;
        let mut has_ten = false;
        
        for card in cards {
            match card.rank() {
                1 => has_ace = true,
                13 => has_king = true,
                12 => has_queen = true,
                11 => has_jack = true,
                10 => has_ten = true,
                _ => {}
            }
        }
        
        has_ace && has_king && has_queen && has_jack && has_ten
    }
    
    // 各ランクの出現回数をカウント
    fn count_ranks(cards: &[Card]) -> RankCountMap {
        let mut rank_counts = HashMap::new();
        for card in cards {
            *rank_counts.entry(card.rank()).or_insert(0) += 1;
        }
        rank_counts
    }
    
    // 出現回数に基づいて役を判定
    fn evaluate_by_counts(rank_counts: RankCountMap, cards: &[Card]) -> HandStrength {
        let mut pairs = Vec::new();
        let mut three_of_a_kind = None;
        let mut four_of_a_kind = None;
        
        for (&rank, &count) in &rank_counts {
            match count {
                2 => pairs.push(rank),
                3 => three_of_a_kind = Some(rank),
                4 => four_of_a_kind = Some(rank),
                _ => {}
            }
        }
        
        // 役に関与しないカードの値（キッカー）を取得
        pairs.sort_by(|a, b| b.cmp(a)); // 降順ソート
        
        if let Some(four) = four_of_a_kind {
            let mut values = vec![four];
            Self::add_kickers(&mut values, &rank_counts, &[four], 1);
            return HandStrength::new(HandRank::FourOfAKind, values);
        }
        
        if let Some(three) = three_of_a_kind {
            if !pairs.is_empty() {
                return HandStrength::new(HandRank::FullHouse, vec![three, pairs[0]]);
            }
            
            let mut values = vec![three];
            Self::add_kickers(&mut values, &rank_counts, &[three], 2);
            return HandStrength::new(HandRank::ThreeOfAKind, values);
        }
        
        if pairs.len() >= 2 {
            let mut values = vec![pairs[0], pairs[1]];
            Self::add_kickers(&mut values, &rank_counts, &[pairs[0], pairs[1]], 1);
            return HandStrength::new(HandRank::TwoPair, values);
        }
        
        if pairs.len() == 1 {
            let mut values = vec![pairs[0]];
            Self::add_kickers(&mut values, &rank_counts, &[pairs[0]], 3);
            return HandStrength::new(HandRank::OnePair, values);
        }
        
        // ハイカード
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
        ranks.sort_by(|a, b| b.cmp(a)); // 降順ソート
        
        // A（1）を14として扱う
        for i in 0..ranks.len() {
            if ranks[i] == 1 {
                ranks[i] = 14;
            }
        }
        
        ranks.sort_by(|a, b| b.cmp(a)); // 再度ソート
        ranks.truncate(5); // 最大5枚まで
        
        HandStrength::new(HandRank::HighCard, ranks)
    }
    
    // キッカーを追加
    fn add_kickers(values: &mut Vec<u8>, rank_counts: &HashMap<u8, u8>, used_ranks: &[u8], count: usize) {
        let mut kickers: Vec<u8> = rank_counts
            .iter()
            .filter(|(&rank, _)| !used_ranks.contains(&rank))
            .map(|(&rank, _)| if rank == 1 { 14 } else { rank })
            .collect();
        
        kickers.sort_by(|a, b| b.cmp(a)); // 降順ソート
        kickers.truncate(count);
        
        values.extend(kickers);
    }
    
    // フラッシュのタイブレーク用の値を取得
    fn get_values_for_flush(cards: &[Card]) -> Vec<u8> {
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
        
        // A（1）を14として扱う
        for i in 0..ranks.len() {
            if ranks[i] == 1 {
                ranks[i] = 14;
            }
        }
        
        ranks.sort_by(|a, b| b.cmp(a)); // 降順ソート
        ranks.truncate(5); // 最大5枚まで
        
        ranks
    }
    
    // 最高ランクのカード値を取得（Aは14として扱う）
    fn get_high_card_value(cards: &[Card]) -> u8 {
        let mut high = 0;
        
        for card in cards {
            let rank = if card.rank() == 1 { 14 } else { card.rank() };
            if rank > high {
                high = rank;
            }
        }
        
        high
    }
} 