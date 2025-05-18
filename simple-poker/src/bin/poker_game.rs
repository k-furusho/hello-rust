use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    fn all() -> [Suit; 4] {
        [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_str = match self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        write!(f, "{}", suit_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    suit: Suit,
    rank: u8,
}

impl Card {
    fn new(suit: Suit, rank: u8) -> Result<Self, &'static str> {
        if rank < 1 || rank > 13 {
            return Err("無効なカードランクです");
        }
        Ok(Self { suit, rank })
    }
    
    fn suit(&self) -> Suit {
        self.suit
    }
    
    fn rank(&self) -> u8 {
        self.rank
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self.rank {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n => n.to_string(),
        };
        write!(f, "{}{}", self.suit, rank_str)
    }
}

#[derive(Debug, Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Result<Self, &'static str> {
        let mut cards = Vec::with_capacity(52);
        
        for &suit in Suit::all().iter() {
            for rank in 1..=13 {
                if let Ok(card) = Card::new(suit, rank) {
                    cards.push(card);
                } else {
                    return Err("デッキの初期化中にエラーが発生しました");
                }
            }
        }
        
        Ok(Self { cards })
    }
    
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }
    
    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    
    fn draw_multiple(&mut self, count: usize) -> Vec<Card> {
        let mut drawn = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.draw() {
                drawn.push(card);
            } else {
                break;
            }
        }
        drawn
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Self { cards: Vec::with_capacity(5) }
    }
    
    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        // ランク順にソート
        self.cards.sort_by_key(|card| card.rank());
    }
    
    fn replace_card(&mut self, index: usize, new_card: Card) -> Result<Card, &'static str> {
        if index >= self.cards.len() {
            return Err("無効なカードインデックスです");
        }
        
        let old_card = self.cards[index];
        self.cards[index] = new_card;
        self.cards.sort_by_key(|card| card.rank());
        
        Ok(old_card)
    }
    
    fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--手札--")?;
        for (i, card) in self.cards.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, card)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
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

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

struct Player {
    name: String,
    hand: Hand,
    chips: u32,
    current_bet: u32,
    is_folded: bool,
    is_all_in: bool,
}

impl Player {
    fn new(name: String, initial_chips: u32) -> Self {
        Self {
            name,
            hand: Hand::new(),
            chips: initial_chips,
            current_bet: 0,
            is_folded: false,
            is_all_in: false,
        }
    }
    
    fn place_bet(&mut self, amount: u32) -> u32 {
        if amount > self.chips {
            let bet = self.chips;
            self.chips = 0;
            self.is_all_in = true;
            self.current_bet += bet;
            bet
        } else {
            self.chips -= amount;
            self.current_bet += amount;
            if self.chips == 0 {
                self.is_all_in = true;
            }
            amount
        }
    }
    
    fn fold(&mut self) {
        self.is_folded = true;
    }
    
    fn reset_bet(&mut self) {
        self.current_bet = 0;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}チップ){}{}",
            self.name,
            self.chips,
            if self.is_all_in { " [オールイン]" } else { "" },
            if self.is_folded { " [フォールド]" } else { "" }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BetAction {
    Fold,   // 降りる
    Check,  // 様子見（現在のベットと同額の場合）
    Call,   // コール（現在のベット額に合わせる）
    Raise,  // レイズ（ベット額を上げる）
    AllIn,  // オールイン（全チップを賭ける）
}

impl fmt::Display for BetAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action_str = match self {
            BetAction::Fold => "フォールド（降りる）",
            BetAction::Check => "チェック（様子見）",
            BetAction::Call => "コール（同額を賭ける）",
            BetAction::Raise => "レイズ（金額を上げる）",
            BetAction::AllIn => "オールイン（全てのチップを賭ける）",
        };
        write!(f, "{}", action_str)
    }
}

struct PokerGame {
    players: Vec<Player>,
    deck: Deck,
    pot: u32,
    current_player_index: usize,
    current_bet: u32,
    hand_size: usize,
    big_blind: u32,
    small_blind: u32,
}

impl PokerGame {
    fn new(big_blind: u32, small_blind: u32) -> Result<Self, &'static str> {
        Ok(Self {
            players: Vec::new(),
            deck: Deck::new()?,
            pot: 0,
            current_player_index: 0,
            current_bet: 0,
            hand_size: 5,  // ファイブカードドロー
            big_blind,
            small_blind,
        })
    }
    
    fn add_player(&mut self, name: String, chips: u32) {
        self.players.push(Player::new(name, chips));
    }
    
    fn deal_cards(&mut self) {
        self.deck = Deck::new().expect("デッキの作成に失敗しました");
        self.deck.shuffle();
        
        for player in &mut self.players {
            player.hand = Hand::new();
            let cards = self.deck.draw_multiple(self.hand_size);
            for card in cards {
                player.hand.add_card(card);
            }
        }
    }
    
    fn post_blinds(&mut self) {
        if self.players.len() < 2 {
            println!("ブラインドを投入するには最低2人のプレイヤーが必要です");
            return;
        }
        
        // スモールブラインド（最初のプレイヤー）
        let small_blind_amount = self.players[0].place_bet(self.small_blind);
        self.pot += small_blind_amount;
        println!("{} がスモールブラインド {}チップを投入", self.players[0].name, small_blind_amount);
        
        // ビッグブラインド（2番目のプレイヤー）
        let big_blind_amount = self.players[1].place_bet(self.big_blind);
        self.pot += big_blind_amount;
        println!("{} がビッグブラインド {}チップを投入", self.players[1].name, big_blind_amount);
        
        self.current_bet = self.big_blind;
        self.current_player_index = 2 % self.players.len(); // 3番目のプレイヤーから開始
    }
    
    fn get_available_actions(&self, player_index: usize) -> Vec<BetAction> {
        let mut actions = Vec::new();
        
        if player_index >= self.players.len() {
            return actions;
        }
        
        let player = &self.players[player_index];
        
        // フォールド済みやオールインのプレイヤーは何もできない
        if player.is_folded || player.is_all_in {
            return actions;
        }
        
        // フォールドは常に可能
        actions.push(BetAction::Fold);
        
        // 現在のベットに対する追加ベット額を計算
        let call_amount = self.current_bet.saturating_sub(player.current_bet);
        
        // チップがない場合はフォールドのみ
        if player.chips == 0 {
            return actions;
        }
        
        // 現在のベットがゼロか、プレイヤーが既に最大ベット額を出している場合はチェック可能
        if self.current_bet == 0 || self.current_bet == player.current_bet {
            actions.push(BetAction::Check);
        }
        
        // プレイヤーが現在のベットをコールできるなら
        if call_amount > 0 && player.chips >= call_amount {
            actions.push(BetAction::Call);
        }
        
        // レイズが可能（現在のベット額+最小ベット額以上のチップがある場合）
        let min_raise = self.big_blind;
        if player.chips >= call_amount + min_raise {
            actions.push(BetAction::Raise);
        }
        
        // オールインは常に可能（ただしチップがある場合のみ）
        if player.chips > 0 {
            actions.push(BetAction::AllIn);
        }
        
        actions
    }
    
    fn execute_action(&mut self, action: BetAction, bet_amount: Option<u32>) -> Result<(), &'static str> {
        let player_index = self.current_player_index;
        if player_index >= self.players.len() {
            return Err("無効なプレイヤーインデックスです");
        }
        
        let available_actions = self.get_available_actions(player_index);
        if !available_actions.contains(&action) {
            return Err("そのアクションは現在実行できません");
        }
        
        let player = &mut self.players[player_index];
        
        match action {
            BetAction::Fold => {
                player.fold();
                println!("{} がフォールドしました", player.name);
            },
            BetAction::Check => {
                println!("{} がチェックしました", player.name);
            },
            BetAction::Call => {
                let call_amount = self.current_bet.saturating_sub(player.current_bet);
                if call_amount > 0 {
                    let amount_bet = player.place_bet(call_amount);
                    self.pot += amount_bet;
                    println!("{} が {}チップでコールしました", player.name, amount_bet);
                } else {
                    println!("{} がチェックしました", player.name);
                }
            },
            BetAction::Raise => {
                if let Some(raise_to) = bet_amount {
                    let call_amount = self.current_bet.saturating_sub(player.current_bet);
                    let min_raise = self.current_bet + self.big_blind;
                    
                    if raise_to < min_raise {
                        return Err("レイズは現在のベット額+最小ベット額以上でなければなりません");
                    }
                    
                    let additional_bet = raise_to.saturating_sub(player.current_bet);
                    if additional_bet > player.chips {
                        return Err("そのレイズに必要なチップが足りません");
                    }
                    
                    let amount_bet = player.place_bet(additional_bet);
                    self.pot += amount_bet;
                    self.current_bet = player.current_bet;
                    
                    println!("{} が {}チップにレイズしました", player.name, self.current_bet);
                } else {
                    return Err("レイズにはベット額を指定する必要があります");
                }
            },
            BetAction::AllIn => {
                let amount_bet = player.place_bet(player.chips);
                self.pot += amount_bet;
                
                if player.current_bet > self.current_bet {
                    self.current_bet = player.current_bet;
                }
                
                println!("{} が {}チップでオールインしました", player.name, amount_bet);
            },
        }
        
        Ok(())
    }
    
    fn next_player(&mut self) -> bool {
        let active_players = self.players.iter()
            .filter(|p| !p.is_folded && !p.is_all_in)
            .count();
            
        if active_players <= 1 {
            return false;
        }
        
        // ベット額が全員一致しているかチェック
        let all_matched = self.players.iter()
            .filter(|p| !p.is_folded && !p.is_all_in)
            .all(|p| p.current_bet == self.current_bet);
            
        if all_matched && self.current_player_index == self.players.len() - 1 {
            return false;
        }
        
        // 次のアクティブなプレイヤーを探す
        let starting = (self.current_player_index + 1) % self.players.len();
        let mut index = starting;
        
        loop {
            if !self.players[index].is_folded && !self.players[index].is_all_in {
                self.current_player_index = index;
                return true;
            }
            
            index = (index + 1) % self.players.len();
            if index == starting {
                // 一周してしまった場合
                return false;
            }
        }
    }
    
    fn allow_exchange_cards(&mut self) {
        println!("\n-- カード交換フェーズ --");
        
        for (index, player) in self.players.iter_mut().enumerate() {
            if player.is_folded {
                continue;
            }
            
            println!("\n{} の番です", player.name);
            println!("{}", player.hand);
            
            println!("交換したいカードの番号を入力してください（スペース区切り、何も入力せずEnterでスキップ）");
            let input = get_string("");
            
            if input.trim().is_empty() {
                println!("{} はカードを交換しませんでした", player.name);
                continue;
            }
            
            let indices: Vec<usize> = input
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .filter(|&n| n >= 1 && n <= self.hand_size)
                .map(|n| n - 1) // 0ベースに変換
                .collect();
                
            if indices.is_empty() {
                println!("有効なカードが選択されていません");
                continue;
            }
            
            // カードを交換
            for &idx in &indices {
                if let Some(new_card) = self.deck.draw() {
                    if let Ok(old_card) = player.hand.replace_card(idx, new_card) {
                        println!("カード {} を {} に交換しました", old_card, new_card);
                    }
                } else {
                    println!("デッキにカードが残っていません");
                    break;
                }
            }
            
            println!("\n交換後の手札:");
            println!("{}", player.hand);
        }
    }
    
    fn showdown(&self) -> Option<usize> {
        println!("\n-- ショーダウン --");
        
        let active_players: Vec<(usize, &Player)> = self.players.iter()
            .enumerate()
            .filter(|(_, p)| !p.is_folded)
            .collect();
            
        if active_players.len() == 1 {
            // 1人だけ残っている場合は自動的に勝者
            println!("{} の勝利！", active_players[0].1.name);
            return Some(active_players[0].0);
        }
        
        let mut best_rank = HandRank::HighCard;
        let mut best_player_idx = 0;
        
        for (idx, player) in active_players {
            println!("\n{} の手札:", player.name);
            println!("{}", player.hand);
            
            let rank = evaluate_hand(player.hand.cards());
            println!("役: {}", rank);
            
            if rank > best_rank {
                best_rank = rank;
                best_player_idx = idx;
            }
        }
        
        println!("\n{} が {} で勝利しました！", self.players[best_player_idx].name, best_rank);
        Some(best_player_idx)
    }
    
    fn betting_round(&mut self) -> bool {
        println!("\n-- ベッティングラウンド --");
        println!("ポット: {}チップ", self.pot);
        
        let mut round_done = false;
        
        while !round_done {
            // 現在のプレイヤーを取得
            let player_index = self.current_player_index;
            let player = &self.players[player_index];
            
            println!("\n{} の番です ({}, チップ: {}, 現在のベット: {})", 
                    player.name, player, player.chips, player.current_bet);
            
            // 使用可能なアクションを表示
            let actions = self.get_available_actions(player_index);
            if actions.is_empty() {
                println!("アクションできません");
                if !self.next_player() {
                    round_done = true;
                }
                continue;
            }
            
            println!("選択可能なアクション:");
            for (i, action) in actions.iter().enumerate() {
                println!("{}. {}", i + 1, action);
            }
            
            let action_idx = match get_menu_choice(actions.len()) {
                Ok(idx) => idx - 1,
                Err(e) => {
                    println!("エラー: {}", e);
                    continue;
                }
            };
            
            let action = actions[action_idx];
            
            // レイズの場合は金額を入力
            let bet_amount = match action {
                BetAction::Raise => {
                    println!("現在のベット: {}チップ", self.current_bet);
                    println!("最小レイズ額: {}チップ", self.current_bet + self.big_blind);
                    match get_u32("レイズ額") {
                        Ok(amount) => Some(amount),
                        Err(e) => {
                            println!("エラー: {}", e);
                            continue;
                        }
                    }
                },
                _ => None,
            };
            
            // アクションを実行
            if let Err(e) = self.execute_action(action, bet_amount) {
                println!("エラー: {}", e);
                continue;
            }
            
            // 次のプレイヤーへ、またはラウンド終了
            if !self.next_player() {
                round_done = true;
            }
        }
        
        // アクティブなプレイヤーが1人しか残っていない場合はゲーム終了
        let active_players = self.players.iter()
            .filter(|p| !p.is_folded)
            .count();
            
        active_players > 1
    }
    
    fn distribute_pot(&mut self, winner_idx: usize) {
        self.players[winner_idx].chips += self.pot;
        println!("{} が {}チップを獲得しました", self.players[winner_idx].name, self.pot);
        self.pot = 0;
    }
    
    fn reset_for_new_hand(&mut self) {
        // ベット状態をリセット
        for player in &mut self.players {
            player.reset_bet();
            player.is_folded = false;
            player.is_all_in = false;
        }
        
        self.pot = 0;
        self.current_bet = 0;
        self.current_player_index = 0;
        
        // プレイヤーの順番をローテーション
        if !self.players.is_empty() {
            let first_player = self.players.remove(0);
            self.players.push(first_player);
        }
    }
    
    fn show_status(&self) {
        println!("\n-- ゲーム状況 --");
        println!("ポット: {}チップ", self.pot);
        println!("現在のベット: {}チップ", self.current_bet);
        
        println!("\nプレイヤー:");
        for (i, player) in self.players.iter().enumerate() {
            println!("{}. {} - {}チップ{}{}",
                i + 1,
                player.name,
                player.chips,
                if player.is_folded { " [フォールド]" } else { "" },
                if player.is_all_in { " [オールイン]" } else { "" }
            );
        }
    }
}

fn evaluate_hand(cards: &[Card]) -> HandRank {
    if cards.len() != 5 {
        return HandRank::HighCard;
    }
    
    let is_flush = is_flush(cards);
    let is_straight = is_straight(cards);
    let is_royal = is_royal(cards);
    let rank_counts = count_ranks(cards);
    
    match (is_flush, is_straight, is_royal) {
        (true, true, true) => HandRank::RoyalStraightFlush,
        (true, true, false) => HandRank::StraightFlush,
        (true, false, _) => HandRank::Flush,
        (false, true, _) => HandRank::Straight,
        _ => evaluate_by_counts(&rank_counts),
    }
}

fn is_flush(cards: &[Card]) -> bool {
    let first_suit = cards[0].suit();
    cards.iter().all(|card| card.suit() == first_suit)
}

fn is_straight(cards: &[Card]) -> bool {
    let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
    ranks.sort();
    
    // A-2-3-4-5の場合
    if ranks == [1, 2, 3, 4, 5] {
        return true;
    }
    
    // 10-J-Q-K-Aの場合
    if ranks == [1, 10, 11, 12, 13] {
        return true;
    }
    
    // 通常のストレートチェック
    ranks.windows(2).all(|w| w[1] == w[0] + 1)
}

fn is_royal(cards: &[Card]) -> bool {
    let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank()).collect();
    ranks.sort();
    ranks == [1, 10, 11, 12, 13]
}

fn count_ranks(cards: &[Card]) -> HashMap<u8, u8> {
    let mut rank_counts = HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank()).or_insert(0) += 1;
    }
    rank_counts
}

fn evaluate_by_counts(rank_counts: &HashMap<u8, u8>) -> HandRank {
    let mut has_three = false;
    let mut pairs = 0;
    
    for &count in rank_counts.values() {
        if count == 4 {
            return HandRank::FourOfAKind;
        }
        if count == 3 {
            has_three = true;
        }
        if count == 2 {
            pairs += 1;
        }
    }
    
    if has_three && pairs > 0 {
        return HandRank::FullHouse;
    }
    
    if has_three {
        return HandRank::ThreeOfAKind;
    }
    
    match pairs {
        2 => HandRank::TwoPair,
        1 => HandRank::OnePair,
        _ => HandRank::HighCard,
    }
}

fn get_string(prompt: &str) -> String {
    if !prompt.is_empty() {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力の読み取りに失敗しました");
    
    input.trim().to_string()
}

fn get_u32(prompt: &str) -> Result<u32, String> {
    let input = get_string(prompt);
    input.parse::<u32>().map_err(|_| format!("無効な数値です: {}", input))
}

fn get_menu_choice(max: usize) -> Result<usize, String> {
    let choice = match get_string("選択").parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err("数値を入力してください".to_string()),
    };
    
    if choice == 0 || choice > max {
        return Err(format!("1から{}までの数字を入力してください", max));
    }
    
    Ok(choice)
}

fn wait_for_enter() {
    print!("続けるにはEnterキーを押してください...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力の読み取りに失敗しました");
}

fn main() {
    println!("\n====================================");
    println!("      ポーカーゲームへようこそ！     ");
    println!("====================================\n");
    
    // ゲーム設定
    println!("ゲーム設定");
    let small_blind = match get_u32("スモールブラインド額") {
        Ok(amount) => amount,
        Err(_) => {
            println!("無効な額です。デフォルトで5に設定します。");
            5
        }
    };
    
    let big_blind = match get_u32("ビッグブラインド額") {
        Ok(amount) if amount > small_blind => amount,
        _ => {
            println!("無効な額です。スモールブラインドの2倍に設定します。");
            small_blind * 2
        }
    };
    
    let mut game = match PokerGame::new(big_blind, small_blind) {
        Ok(g) => g,
        Err(e) => {
            println!("ゲームの作成に失敗しました: {}", e);
            return;
        }
    };
    
    // プレイヤー追加
    println!("\nプレイヤーを追加（最低2人必要）");
    loop {
        let name = get_string("プレイヤー名（終了するには'q'を入力）");
        if name.to_lowercase() == "q" {
            if game.players.len() < 2 {
                println!("最低2人のプレイヤーが必要です。追加してください。");
                continue;
            }
            break;
        }
        
        let initial_chips = match get_u32("初期チップ数") {
            Ok(chips) => chips,
            Err(_) => {
                println!("無効な額です。デフォルトで500に設定します。");
                500
            }
        };
        
        game.add_player(name, initial_chips);
        println!("プレイヤーを追加しました。現在のプレイヤー数: {}", game.players.len());
        
        if game.players.len() >= 2 {
            println!("'q'を入力して次に進むか、さらにプレイヤーを追加できます。");
        }
    }
    
    // メインゲームループ
    let mut game_continues = true;
    
    while game_continues {
        println!("\n新しいハンドを開始します。");
        game.reset_for_new_hand();
        
        // カードを配る
        game.deal_cards();
        
        // ブラインドを投入
        game.post_blinds();
        
        // 各プレイヤーの手札を表示
        for player in &game.players {
            println!("\n{} の手札:", player.name);
            println!("{}", player.hand);
            wait_for_enter();
        }
        
        // 初回ベッティングラウンド
        let continue_game = game.betting_round();
        
        if continue_game {
            // カード交換フェーズ
            game.allow_exchange_cards();
            
            // 2回目のベッティングラウンド
            let continue_after_exchange = game.betting_round();
            
            if continue_after_exchange {
                // ショーダウン
                if let Some(winner_idx) = game.showdown() {
                    game.distribute_pot(winner_idx);
                }
            } else if let Some(winner_idx) = game.players.iter()
                .enumerate()
                .find(|(_, p)| !p.is_folded)
                .map(|(i, _)| i) {
                println!("\n{} が勝利しました！（他のプレイヤーは全員フォールド）", game.players[winner_idx].name);
                game.distribute_pot(winner_idx);
            }
        } else if let Some(winner_idx) = game.players.iter()
            .enumerate()
            .find(|(_, p)| !p.is_folded)
            .map(|(i, _)| i) {
            println!("\n{} が勝利しました！（他のプレイヤーは全員フォールド）", game.players[winner_idx].name);
            game.distribute_pot(winner_idx);
        }
        
        // プレイヤーの状態を表示
        println!("\n-- ゲーム終了 --");
        for player in &game.players {
            println!("{}: {}チップ", player.name, player.chips);
        }
        
        // 続けるか確認
        println!("\n新しいゲームを始めますか？ (y/n)");
        let continue_input = get_string("");
        game_continues = continue_input.to_lowercase().starts_with('y');
    }
    
    println!("\nポーカーゲームを終了します。お疲れ様でした！");
} 