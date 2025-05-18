use uuid::Uuid;

use super::bet::Pot;
use super::card::Card;
use super::deck::Deck;
use super::player::Player;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVariant {
    FiveCardDraw,
    TexasHoldem,
    Omaha,
}

impl GameVariant {
    pub fn hand_size(&self) -> usize {
        match self {
            GameVariant::FiveCardDraw => 5,
            GameVariant::TexasHoldem => 2,
            GameVariant::Omaha => 4,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            GameVariant::FiveCardDraw => "ファイブカードドロー",
            GameVariant::TexasHoldem => "テキサスホールデム",
            GameVariant::Omaha => "オマハ",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BettingRound {
    PreDraw,   // ファイブカードドローでの最初のベッティングラウンド
    PostDraw,  // ファイブカードドローでの2回目のベッティングラウンド
    PreFlop,   // テキサスホールデム・オマハでの最初のベッティングラウンド
    Flop,      // テキサスホールデム・オマハでのフロップ後のベッティングラウンド
    Turn,      // テキサスホールデム・オマハでのターン後のベッティングラウンド
    River,     // テキサスホールデム・オマハでのリバー後のベッティングラウンド
}

impl BettingRound {
    pub fn next(&self, game_variant: GameVariant) -> Option<Self> {
        match (game_variant, self) {
            (GameVariant::FiveCardDraw, BettingRound::PreDraw) => Some(BettingRound::PostDraw),
            (GameVariant::FiveCardDraw, BettingRound::PostDraw) => None,
            (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::PreFlop) => Some(BettingRound::Flop),
            (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::Flop) => Some(BettingRound::Turn),
            (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::Turn) => Some(BettingRound::River),
            (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::River) => None,
            _ => None,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            BettingRound::PreDraw => "ドロー前",
            BettingRound::PostDraw => "ドロー後",
            BettingRound::PreFlop => "プリフロップ",
            BettingRound::Flop => "フロップ",
            BettingRound::Turn => "ターン",
            BettingRound::River => "リバー",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    NotStarted,
    Dealing,
    Betting,
    Drawing,
    Showdown,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameId(String);

impl GameId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    pub fn from_string(id: String) -> Self {
        Self(id)
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    id: GameId,
    variant: GameVariant,
    players: Vec<Player>,
    deck: Deck,
    pot: Pot,
    community_cards: Vec<Card>,
    current_round: Option<BettingRound>,
    current_phase: GamePhase,
    current_player_index: usize,
    dealer_index: usize,
    small_blind: u32,
    big_blind: u32,
    current_bet: u32,  // 現在のラウンドでの最大ベット額
}

impl Game {
    pub fn new(variant: GameVariant, small_blind: u32, big_blind: u32) -> Result<Self, &'static str> {
        // スモールブラインドがビッグブラインドより大きいとエラー
        if small_blind > big_blind {
            return Err("スモールブラインドはビッグブラインド以下である必要があります");
        }
        
        Ok(Self {
            id: GameId::new(),
            variant,
            players: Vec::new(),
            deck: Deck::new()?,
            pot: Pot::new(),
            community_cards: Vec::new(),
            current_round: None,
            current_phase: GamePhase::NotStarted,
            current_player_index: 0,
            dealer_index: 0,
            small_blind,
            big_blind,
            current_bet: 0,
        })
    }
    
    pub fn id(&self) -> &GameId {
        &self.id
    }
    
    pub fn variant(&self) -> GameVariant {
        self.variant
    }
    
    pub fn add_player(&mut self, player: Player) -> Result<(), &'static str> {
        if self.current_phase != GamePhase::NotStarted {
            return Err("ゲームが既に開始されています");
        }
        
        if self.players.len() >= 10 {
            return Err("プレイヤー数の上限に達しています");
        }
        
        // プレイヤーIDが重複していないか確認
        if self.players.iter().any(|p| p.id() == player.id()) {
            return Err("このプレイヤーIDは既に使用されています");
        }
        
        self.players.push(player);
        Ok(())
    }
    
    pub fn players(&self) -> &[Player] {
        &self.players
    }
    
    pub fn players_mut(&mut self) -> &mut [Player] {
        &mut self.players
    }
    
    pub fn pot(&self) -> &Pot {
        &self.pot
    }
    
    pub fn pot_mut(&mut self) -> &mut Pot {
        &mut self.pot
    }
    
    pub fn community_cards(&self) -> &[Card] {
        &self.community_cards
    }
    
    pub fn current_round(&self) -> Option<BettingRound> {
        self.current_round
    }
    
    pub fn current_phase(&self) -> GamePhase {
        self.current_phase
    }
    
    pub fn current_player_index(&self) -> usize {
        self.current_player_index
    }
    
    pub fn current_player(&self) -> Option<&Player> {
        self.players.get(self.current_player_index)
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
    
    pub fn set_current_bet(&mut self, amount: u32) {
        self.current_bet = amount;
    }
    
    pub fn start_game(&mut self) -> Result<(), &'static str> {
        if self.players.len() < 2 {
            return Err("ゲームを開始するには最低2人のプレイヤーが必要です");
        }
        
        if self.current_phase != GamePhase::NotStarted {
            return Err("ゲームは既に開始されています");
        }
        
        // デッキをシャッフル
        self.deck.shuffle();
        
        // ディーラーを設定（ランダムに）
        self.dealer_index = 0;
        self.players[self.dealer_index].set_dealer(true);
        
        // ゲームバリアントに応じて最初のラウンドを設定
        self.current_round = Some(match self.variant {
            GameVariant::FiveCardDraw => BettingRound::PreDraw,
            GameVariant::TexasHoldem | GameVariant::Omaha => BettingRound::PreFlop,
        });
        
        self.current_phase = GamePhase::Dealing;
        self.current_player_index = self.next_active_player_index(self.dealer_index);
        
        Ok(())
    }
    
    pub fn deal_cards(&mut self) -> Result<(), &'static str> {
        if self.current_phase != GamePhase::Dealing {
            return Err("カードを配るのはDealingフェーズでのみ可能です");
        }
        
        let hand_size = self.variant.hand_size();
        
        // 各プレイヤーに手札を配る
        for player in &mut self.players {
            player.reset_for_new_round();
            let cards = self.deck.draw_multiple(hand_size);
            
            for card in cards {
                player.hand_mut().add_card(card)?;
            }
        }
        
        // バリアントに応じてコミュニティカードを配る（ホールデムとオマハの場合）
        if matches!(self.variant, GameVariant::TexasHoldem | GameVariant::Omaha) {
            self.community_cards.clear();
        }
        
        // フェーズを更新
        self.current_phase = GamePhase::Betting;
        
        Ok(())
    }
    
    pub fn post_blinds(&mut self) -> Result<(), &'static str> {
        if self.current_phase != GamePhase::Betting || self.current_round != Some(BettingRound::PreFlop) {
            return Err("ブラインドはベッティングフェーズのプリフロップでのみ投入可能です");
        }
        
        if self.players.len() < 2 {
            return Err("ブラインドを投入するには最低2人のプレイヤーが必要です");
        }
        
        // スモールブラインドのプレイヤーを特定
        let small_blind_index = self.next_active_player_index(self.dealer_index);
        
        // スモールブラインドを投入
        let small_blind_amount = self.players[small_blind_index].place_bet(self.small_blind)?;
        self.pot.add(small_blind_amount);
        
        // ビッグブラインドのプレイヤーを特定
        let big_blind_index = self.next_active_player_index(small_blind_index);
        
        // ビッグブラインドを投入
        let big_blind_amount = self.players[big_blind_index].place_bet(self.big_blind)?;
        self.pot.add(big_blind_amount);
        
        // 現在のベット額をビッグブラインドに設定
        self.current_bet = self.big_blind;
        
        // アクションを始めるプレイヤーを設定（ビッグブラインドの次のプレイヤー）
        self.current_player_index = self.next_active_player_index(big_blind_index);
        
        Ok(())
    }
    
    // 次の有効なプレイヤーのインデックスを取得
    fn next_active_player_index(&self, from_index: usize) -> usize {
        let mut index = (from_index + 1) % self.players.len();
        while index != from_index {
            let player = &self.players[index];
            if !player.is_folded() && !player.is_all_in() {
                return index;
            }
            index = (index + 1) % self.players.len();
        }
        from_index
    }
    
    // アクティブなプレイヤー数を取得（フォールドやオールインしていないプレイヤー）
    pub fn active_player_count(&self) -> usize {
        self.players.iter().filter(|p| !p.is_folded() && !p.is_all_in()).count()
    }
    
    // ラウンド終了時の処理
    pub fn end_betting_round(&mut self) -> Result<(), &'static str> {
        if self.current_phase != GamePhase::Betting {
            return Err("ベッティングラウンドが進行中ではありません");
        }
        
        // 現在のラウンドを取得
        let current_round = self.current_round.ok_or("現在のラウンドが設定されていません")?;
        
        // プレイヤーのベット額をリセット
        for player in &mut self.players {
            player.reset_bet();
        }
        
        // 次のラウンドがあるかどうかを確認
        match current_round.next(self.variant) {
            Some(next_round) => {
                self.current_round = Some(next_round);
                self.current_bet = 0;
                
                // テキサスホールデムとオマハの場合、コミュニティカードを配る
                match (self.variant, next_round) {
                    (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::Flop) => {
                        // フロップ：3枚のカードを開く
                        for _ in 0..3 {
                            if let Some(card) = self.deck.draw() {
                                self.community_cards.push(card);
                            }
                        }
                    },
                    (GameVariant::TexasHoldem | GameVariant::Omaha, BettingRound::Turn | BettingRound::River) => {
                        // ターンとリバー：それぞれ1枚のカードを開く
                        if let Some(card) = self.deck.draw() {
                            self.community_cards.push(card);
                        }
                    },
                    (GameVariant::FiveCardDraw, BettingRound::PostDraw) => {
                        // ファイブカードドローの場合、カード交換フェーズに移行
                        self.current_phase = GamePhase::Drawing;
                        return Ok(());
                    },
                    _ => {},
                }
                
                // 次のラウンドのアクションを始めるプレイヤーを設定（ディーラーの次のプレイヤー）
                self.current_player_index = self.next_active_player_index(self.dealer_index);
                
            },
            None => {
                // 最後のラウンドが終了した場合、ショーダウンに移行
                self.current_phase = GamePhase::Showdown;
            },
        }
        
        Ok(())
    }
    
    // ファイブカードドローのカード交換処理
    pub fn exchange_cards(&mut self, player_index: usize, card_indices: &[usize]) -> Result<(), &'static str> {
        if self.current_phase != GamePhase::Drawing {
            return Err("カード交換はドローフェーズでのみ可能です");
        }
        
        if player_index >= self.players.len() {
            return Err("無効なプレイヤーインデックスです");
        }
        
        let player = &mut self.players[player_index];
        
        // 指定されたカードを交換
        for &index in card_indices {
            if index >= player.hand().size() {
                return Err("無効なカードインデックスです");
            }
            
            if let Some(new_card) = self.deck.draw() {
                if let Ok(old_card) = player.hand_mut().replace_card(index, new_card) {
                    // 古いカードをデッキに戻してシャッフル（オプション）
                    self.deck.add_card(old_card);
                }
            } else {
                return Err("デッキにカードが残っていません");
            }
        }
        
        // すべてのプレイヤーが交換を終えたかチェック
        self.current_player_index = self.next_active_player_index(player_index);
        
        if self.current_player_index == 0 {
            // すべてのプレイヤーがカード交換を完了したら、次のベッティングラウンドへ
            self.current_phase = GamePhase::Betting;
        }
        
        Ok(())
    }
    
    // ゲームをリセットして新しいハンドを開始する準備
    pub fn reset_for_new_hand(&mut self) -> Result<(), &'static str> {
        // デッキをリセット
        self.deck = Deck::new()?;
        
        // ポットをクリア
        self.pot.clear();
        
        // コミュニティカードをクリア
        self.community_cards.clear();
        
        // プレイヤーの状態をリセット
        for player in &mut self.players {
            player.reset_for_new_game();
        }
        
        // ディーラーボタンを次のプレイヤーに移動
        self.players[self.dealer_index].set_dealer(false);
        self.dealer_index = (self.dealer_index + 1) % self.players.len();
        self.players[self.dealer_index].set_dealer(true);
        
        // ゲームの状態をリセット
        self.current_round = None;
        self.current_phase = GamePhase::NotStarted;
        self.current_bet = 0;
        self.current_player_index = 0;
        
        Ok(())
    }
    
    // デシリアライズのためのファクトリメソッド
    pub fn from_serialized(
        id: GameId,
        variant: GameVariant,
        players: Vec<Player>,
        community_cards: Vec<Card>,
        pot_total: u32,
        current_phase: GamePhase,
        current_round: Option<BettingRound>,
        current_player_index: usize,
        dealer_index: usize,
        small_blind: u32,
        big_blind: u32,
        current_bet: u32,
    ) -> Result<Self, &'static str> {
        let mut game = Self::new(variant, small_blind, big_blind)?;
        
        // IDの設定
        game.id = id;
        
        // プレイヤーの追加
        game.players = players;
        
        // コミュニティカードのセット
        game.community_cards = community_cards;
        
        // ポットの設定
        game.pot = Pot::new();
        game.pot_mut().add(pot_total);
        
        // 各種ステータスの設定
        game.current_phase = current_phase;
        game.current_round = current_round;
        game.current_player_index = current_player_index;
        game.dealer_index = dealer_index;
        game.current_bet = current_bet;
        
        // ディーラーフラグを設定
        if game.players.len() > dealer_index {
            for (i, player) in game.players.iter_mut().enumerate() {
                player.set_dealer(i == dealer_index);
            }
        }
        
        Ok(game)
    }
} 