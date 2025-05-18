use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::model::game::{Game, GameId, GameVariant, GamePhase, BettingRound};
use crate::domain::model::player::{Player, PlayerId};
use crate::domain::model::card::Card;
use crate::domain::model::bet::Pot;
use crate::domain::repository::game_repository::GameRepository;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializableGame {
    id: String,
    variant: String,
    players: Vec<SerializablePlayer>,
    community_cards: Vec<SerializableCard>,
    pot: u32,
    current_phase: String,
    current_round: Option<String>,
    current_player_index: usize,
    dealer_index: usize,
    small_blind: u32,
    big_blind: u32,
    current_bet: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializablePlayer {
    id: String,
    name: String,
    chips: u32,
    hand: Vec<SerializableCard>,
    current_bet: u32,
    is_folded: bool,
    is_all_in: bool,
    is_dealer: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializableCard {
    suit: String,
    rank: u8,
}

impl From<&Game> for SerializableGame {
    fn from(game: &Game) -> Self {
        Self {
            id: game.id().value().to_string(),
            variant: match game.variant() {
                GameVariant::FiveCardDraw => "FiveCardDraw".to_string(),
                GameVariant::TexasHoldem => "TexasHoldem".to_string(),
                GameVariant::Omaha => "Omaha".to_string(),
            },
            players: game.players().iter().map(SerializablePlayer::from).collect(),
            community_cards: game.community_cards().iter().map(SerializableCard::from).collect(),
            pot: game.pot().total(),
            current_phase: match game.current_phase() {
                GamePhase::NotStarted => "NotStarted".to_string(),
                GamePhase::Dealing => "Dealing".to_string(),
                GamePhase::Betting => "Betting".to_string(),
                GamePhase::Drawing => "Drawing".to_string(),
                GamePhase::Showdown => "Showdown".to_string(),
                GamePhase::Complete => "Complete".to_string(),
            },
            current_round: game.current_round().map(|r| match r {
                BettingRound::PreDraw => "PreDraw".to_string(),
                BettingRound::PostDraw => "PostDraw".to_string(),
                BettingRound::PreFlop => "PreFlop".to_string(),
                BettingRound::Flop => "Flop".to_string(),
                BettingRound::Turn => "Turn".to_string(),
                BettingRound::River => "River".to_string(),
            }),
            current_player_index: game.current_player_index(),
            dealer_index: game.dealer_index(),
            small_blind: game.small_blind(),
            big_blind: game.big_blind(),
            current_bet: game.current_bet(),
        }
    }
}

impl From<&Player> for SerializablePlayer {
    fn from(player: &Player) -> Self {
        Self {
            id: player.id().value().to_string(),
            name: player.name().to_string(),
            chips: player.chips(),
            hand: player.hand().cards().iter().map(SerializableCard::from).collect(),
            current_bet: player.current_bet(),
            is_folded: player.is_folded(),
            is_all_in: player.is_all_in(),
            is_dealer: player.is_dealer(),
        }
    }
}

impl From<&Card> for SerializableCard {
    fn from(card: &Card) -> Self {
        Self {
            suit: match card.suit() {
                crate::domain::model::card::Suit::Club => "Club".to_string(),
                crate::domain::model::card::Suit::Diamond => "Diamond".to_string(),
                crate::domain::model::card::Suit::Heart => "Heart".to_string(),
                crate::domain::model::card::Suit::Spade => "Spade".to_string(),
            },
            rank: card.rank(),
        }
    }
}

// 注意: 完全な復元実装はGame構造体に依存するため、ここでは概略のみ
// 実際の実装にはデッキの状態なども含めた複雑な変換が必要

pub struct FileGameRepository {
    directory: PathBuf,
    games: HashMap<String, Game>,
}

impl FileGameRepository {
    pub fn new<P: AsRef<Path>>(directory: P) -> Result<Self, String> {
        let directory = directory.as_ref().to_path_buf();
        
        // ディレクトリが存在しない場合は作成
        if !directory.exists() {
            fs::create_dir_all(&directory)
                .map_err(|e| format!("ディレクトリの作成に失敗しました: {}", e))?;
        }
        
        let repo = Self {
            directory,
            games: HashMap::new(),
        };
        
        Ok(repo)
    }
    
    fn get_game_path(&self, id: &GameId) -> PathBuf {
        self.directory.join(format!("game_{}.json", id.value()))
    }
    
    fn save_game(&self, game: &Game) -> Result<(), String> {
        let serializable = SerializableGame::from(game);
        let json = serde_json::to_string_pretty(&serializable)
            .map_err(|e| format!("JSONへの変換に失敗しました: {}", e))?;
            
        let game_path = self.get_game_path(game.id());
        let mut file = File::create(&game_path)
            .map_err(|e| format!("ファイルの作成に失敗しました: {}", e))?;
            
        file.write_all(json.as_bytes())
            .map_err(|e| format!("ファイルの書き込みに失敗しました: {}", e))?;
            
        Ok(())
    }
    
    fn load_game(&self, id: &GameId) -> Result<Game, String> {
        let game_path = self.get_game_path(id);
        
        if !game_path.exists() {
            return Err(format!("ゲーム {} が見つかりません", id.value()));
        }
        
        let mut file = File::open(&game_path)
            .map_err(|e| format!("ファイルを開けませんでした: {}", e))?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("ファイルの読み込みに失敗しました: {}", e))?;
            
        let _serializable: SerializableGame = serde_json::from_str(&contents)
            .map_err(|e| format!("JSONのパースに失敗しました: {}", e))?;
            
        // SerializableGameからGameへの変換は複雑なため、実際の実装では
        // ドメインモデルを完全に再構築する必要があります。
        // ここでは簡略化のためエラーを返します。
        
        Err("ゲームの復元は現在サポートされていません。実装を完了してください。".to_string())
    }
}

impl GameRepository for FileGameRepository {
    fn save(&mut self, game: &Game) -> Result<(), String> {
        self.save_game(game)
    }
    
    fn find_by_id(&self, id: &GameId) -> Option<Game> {
        match self.load_game(id) {
            Ok(game) => Some(game),
            Err(_) => None,
        }
    }
    
    fn find_all(&self) -> Vec<Game> {
        let mut games = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    let filename = path.file_name()?.to_string_lossy();
                    
                    if filename.starts_with("game_") && filename.ends_with(".json") {
                        let id_str = filename.strip_prefix("game_")?.strip_suffix(".json")?;
                        let id = GameId::from_string(id_str.to_string());
                        
                        if let Ok(game) = self.load_game(&id) {
                            games.push(game);
                        }
                    }
                }
            }
        }
        
        games
    }
    
    fn delete(&mut self, id: &GameId) -> Result<(), String> {
        let game_path = self.get_game_path(id);
        
        if !game_path.exists() {
            return Err(format!("ゲーム {} が見つかりません", id.value()));
        }
        
        fs::remove_file(&game_path)
            .map_err(|e| format!("ファイルの削除に失敗しました: {}", e))?;
            
        Ok(())
    }
} 