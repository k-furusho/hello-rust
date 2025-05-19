use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::model::game::{Game, GameId, GameVariant, GamePhase, BettingRound};
use crate::domain::model::player::{Player, PlayerId};
use crate::domain::model::card::{Card, Suit};
use crate::domain::repository::game_repository::GameRepository;

// シリアライズ用のモデルを別モジュールに分離
mod serializable {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SerializableGame {
        pub id: String,
        pub variant: String,
        pub players: Vec<SerializablePlayer>,
        pub community_cards: Vec<SerializableCard>,
        pub pot: u32,
        pub current_phase: String,
        pub current_round: Option<String>,
        pub current_player_index: usize,
        pub dealer_index: usize,
        pub small_blind: u32,
        pub big_blind: u32,
        pub current_bet: u32,
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SerializablePlayer {
        pub id: String,
        pub name: String,
        pub chips: u32,
        pub hand: Vec<SerializableCard>,
        pub current_bet: u32,
        pub is_folded: bool,
        pub is_all_in: bool,
        pub is_dealer: bool,
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SerializableCard {
        pub suit: String,
        pub rank: u8,
    }
}

// シリアライザークラス - ドメインモデルとシリアライズモデル間の変換を担当
struct GameSerializer;

impl GameSerializer {
    // ゲームをシリアライズ可能な形式に変換
    fn to_serializable(game: &Game) -> serializable::SerializableGame {
        serializable::SerializableGame {
            id: game.id().value().to_string(),
            variant: match game.variant() {
                GameVariant::FiveCardDraw => "FiveCardDraw".to_string(),
                GameVariant::TexasHoldem => "TexasHoldem".to_string(),
                GameVariant::Omaha => "Omaha".to_string(),
            },
            players: game.players().iter().map(Self::player_to_serializable).collect(),
            community_cards: game.community_cards().iter().map(Self::card_to_serializable).collect(),
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
    
    // プレイヤーをシリアライズ可能な形式に変換
    fn player_to_serializable(player: &Player) -> serializable::SerializablePlayer {
        serializable::SerializablePlayer {
            id: player.id().value().to_string(),
            name: player.name().to_string(),
            chips: player.chips(),
            hand: player.hand().cards().iter().map(Self::card_to_serializable).collect(),
            current_bet: player.current_bet(),
            is_folded: player.is_folded(),
            is_all_in: player.is_all_in(),
            is_dealer: player.is_dealer(),
        }
    }
    
    // カードをシリアライズ可能な形式に変換
    fn card_to_serializable(card: &Card) -> serializable::SerializableCard {
        serializable::SerializableCard {
            suit: match card.suit() {
                Suit::Club => "Club".to_string(),
                Suit::Diamond => "Diamond".to_string(),
                Suit::Heart => "Heart".to_string(),
                Suit::Spade => "Spade".to_string(),
            },
            rank: card.rank(),
        }
    }
    
    // シリアライズ済みのゲームからゲームモデルに変換
    fn from_serializable(serializable: serializable::SerializableGame) -> Result<Game, String> {
        // バリアントの復元
        let variant = match serializable.variant.as_str() {
            "FiveCardDraw" => GameVariant::FiveCardDraw,
            "TexasHoldem" => GameVariant::TexasHoldem,
            "Omaha" => GameVariant::Omaha,
            _ => return Err(format!("不明なゲームバリアント: {}", serializable.variant)),
        };
        
        // フェーズの復元
        let phase = match serializable.current_phase.as_str() {
            "NotStarted" => GamePhase::NotStarted,
            "Dealing" => GamePhase::Dealing,
            "Betting" => GamePhase::Betting,
            "Drawing" => GamePhase::Drawing,
            "Showdown" => GamePhase::Showdown,
            "Complete" => GamePhase::Complete,
            _ => return Err(format!("不明なゲームフェーズ: {}", serializable.current_phase)),
        };
        
        // ラウンドの復元
        let round = if let Some(round_str) = serializable.current_round {
            match round_str.as_str() {
                "PreDraw" => Some(BettingRound::PreDraw),
                "PostDraw" => Some(BettingRound::PostDraw),
                "PreFlop" => Some(BettingRound::PreFlop),
                "Flop" => Some(BettingRound::Flop),
                "Turn" => Some(BettingRound::Turn),
                "River" => Some(BettingRound::River),
                _ => return Err(format!("不明なベッティングラウンド: {}", round_str)),
            }
        } else {
            None
        };
        
        // プレイヤーの復元
        let players = serializable.players.into_iter()
            .map(Self::player_from_serializable)
            .collect::<Result<Vec<_>, _>>()?;
        
        // コミュニティカードの復元
        let community_cards = serializable.community_cards.iter()
            .map(Self::card_from_serializable)
            .collect::<Result<Vec<_>, _>>()?;
        
        // GameIDを復元
        let id = GameId::from_string(serializable.id);
        
        // ゲームを復元（デシリアライズのファクトリメソッドを使用）
        Game::from_serialized(
            id,
            variant,
            players,
            community_cards,
            serializable.pot,
            phase,
            round,
            serializable.current_player_index,
            serializable.dealer_index,
            serializable.small_blind,
            serializable.big_blind,
            serializable.current_bet,
        ).map_err(|e| format!("ゲームの復元に失敗しました: {}", e))
    }
    
    // シリアライズ済みのプレイヤーからプレイヤーモデルに変換
    fn player_from_serializable(serializable: serializable::SerializablePlayer) -> Result<Player, String> {
        // プレイヤーIDを復元
        let player_id = PlayerId::from_string(serializable.id);
        
        // 手札を復元
        let cards = serializable.hand.iter()
            .map(Self::card_from_serializable)
            .collect::<Result<Vec<_>, _>>()?;
        
        // プレイヤーを復元（デシリアライズのファクトリメソッドを使用）
        Player::from_serialized(
            player_id,
            serializable.name,
            serializable.chips,
            cards,
            serializable.current_bet,
            serializable.is_folded,
            serializable.is_all_in,
            serializable.is_dealer,
        ).map_err(|e| format!("プレイヤーの復元に失敗しました: {}", e))
    }
    
    // シリアライズ済みのカードからカードモデルに変換
    fn card_from_serializable(serializable: &serializable::SerializableCard) -> Result<Card, String> {
        let suit = match serializable.suit.as_str() {
            "Club" => Suit::Club,
            "Diamond" => Suit::Diamond,
            "Heart" => Suit::Heart,
            "Spade" => Suit::Spade,
            _ => return Err(format!("不明なスート: {}", serializable.suit)),
        };
        
        Card::new(suit, serializable.rank)
            .map_err(|e| format!("カードの作成に失敗しました: {}", e))
    }
}

#[derive(Clone)]
pub struct FileGameRepository {
    directory: PathBuf,
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
        };
        
        Ok(repo)
    }
    
    fn get_game_path(&self, id: &GameId) -> PathBuf {
        self.directory.join(format!("game_{}.json", id.value()))
    }
    
    fn save_game(&self, game: &Game) -> Result<(), String> {
        // ゲームをシリアライズ可能な形式に変換
        let serializable = GameSerializer::to_serializable(game);
        
        // JSONに変換
        let json = serde_json::to_string_pretty(&serializable)
            .map_err(|e| format!("JSONへの変換に失敗しました: {}", e))?;
            
        // ファイルに保存
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
        
        // ファイルを読み込み
        let mut file = File::open(&game_path)
            .map_err(|e| format!("ファイルを開けませんでした: {}", e))?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("ファイルの読み込みに失敗しました: {}", e))?;
            
        // JSONをデシリアライズ
        let serializable: serializable::SerializableGame = serde_json::from_str(&contents)
            .map_err(|e| format!("JSONのパースに失敗しました: {}", e))?;
            
        // ゲームオブジェクトに変換
        GameSerializer::from_serializable(serializable)
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
                    if let Some(filename_os) = path.file_name() {
                        let filename = filename_os.to_string_lossy();
                        
                        if filename.starts_with("game_") && filename.ends_with(".json") {
                            // game_とjsonの間の部分を取得
                            if let Some(id_str) = filename.strip_prefix("game_") {
                                if let Some(id_str) = id_str.strip_suffix(".json") {
                                    let id = GameId::from_string(id_str.to_string());
                                    
                                    if let Ok(game) = self.load_game(&id) {
                                        games.push(game);
                                    }
                                }
                            }
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