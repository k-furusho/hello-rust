use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use chrono::DateTime;

use crate::domain::model::game::GameId;
use crate::domain::model::player::PlayerId;
use crate::domain::repository::game_history_repository::{GameHistoryEntry, GameHistoryRepository};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializableGameHistoryEntry {
    game_id: String,
    timestamp: String,
    winner_ids: Vec<String>,
    pot_amount: u32,
    variant: String,
    player_count: usize,
}

impl From<&GameHistoryEntry> for SerializableGameHistoryEntry {
    fn from(entry: &GameHistoryEntry) -> Self {
        Self {
            game_id: entry.game_id.value().to_string(),
            timestamp: entry.timestamp.to_rfc3339(),
            winner_ids: entry.winner_ids.iter().map(|id| id.value().to_string()).collect(),
            pot_amount: entry.pot_amount,
            variant: entry.variant.clone(),
            player_count: entry.player_count,
        }
    }
}

impl TryFrom<SerializableGameHistoryEntry> for GameHistoryEntry {
    type Error = String;
    
    fn try_from(serializable: SerializableGameHistoryEntry) -> Result<Self, Self::Error> {
        let game_id = GameId::from_string(serializable.game_id);
        let timestamp = DateTime::parse_from_rfc3339(&serializable.timestamp)
            .map_err(|e| format!("日時のパースエラー: {}", e))?
            .into();
        
        let winner_ids = serializable.winner_ids.into_iter()
            .map(PlayerId::from_string)
            .collect();
        
        Ok(Self {
            game_id,
            timestamp,
            winner_ids,
            pot_amount: serializable.pot_amount,
            variant: serializable.variant,
            player_count: serializable.player_count,
        })
    }
}

pub struct FileGameHistoryRepository {
    directory: PathBuf,
    entries: HashMap<String, GameHistoryEntry>,
}

impl FileGameHistoryRepository {
    pub fn new<P: AsRef<Path>>(directory: P) -> Result<Self, String> {
        let directory = directory.as_ref().to_path_buf();
        
        // ディレクトリが存在しない場合は作成
        if !directory.exists() {
            fs::create_dir_all(&directory)
                .map_err(|e| format!("ディレクトリの作成に失敗しました: {}", e))?;
        }
        
        let mut repo = Self {
            directory,
            entries: HashMap::new(),
        };
        
        // 既存のファイルを読み込む
        repo.load_entries()?;
        
        Ok(repo)
    }
    
    fn load_entries(&mut self) -> Result<(), String> {
        self.entries.clear();
        
        let entries_path = self.directory.join("game_history.json");
        if !entries_path.exists() {
            return Ok(());
        }
        
        let mut file = File::open(&entries_path)
            .map_err(|e| format!("ファイルを開けませんでした: {}", e))?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("ファイルの読み込みに失敗しました: {}", e))?;
            
        if contents.is_empty() {
            return Ok(());
        }
        
        let serializable_entries: Vec<SerializableGameHistoryEntry> = serde_json::from_str(&contents)
            .map_err(|e| format!("JSONのパースに失敗しました: {}", e))?;
            
        for serializable in serializable_entries {
            let entry = GameHistoryEntry::try_from(serializable)?;
            let key = format!("{}_{}", entry.game_id.value(), entry.timestamp.to_rfc3339());
            self.entries.insert(key, entry);
        }
        
        Ok(())
    }
    
    fn save_entries(&self) -> Result<(), String> {
        let serializable_entries: Vec<SerializableGameHistoryEntry> = self.entries.values()
            .map(|entry| SerializableGameHistoryEntry::from(entry))
            .collect();
            
        let json = serde_json::to_string_pretty(&serializable_entries)
            .map_err(|e| format!("JSONへの変換に失敗しました: {}", e))?;
            
        let entries_path = self.directory.join("game_history.json");
        let mut file = File::create(&entries_path)
            .map_err(|e| format!("ファイルの作成に失敗しました: {}", e))?;
            
        file.write_all(json.as_bytes())
            .map_err(|e| format!("ファイルの書き込みに失敗しました: {}", e))?;
            
        Ok(())
    }
}

impl GameHistoryRepository for FileGameHistoryRepository {
    fn save(&mut self, entry: &GameHistoryEntry) -> Result<(), String> {
        let key = format!("{}_{}", entry.game_id.value(), entry.timestamp.to_rfc3339());
        self.entries.insert(key, entry.clone());
        self.save_entries()
    }
    
    fn find_by_game_id(&self, game_id: &GameId) -> Option<GameHistoryEntry> {
        self.entries.values()
            .filter(|e| e.game_id.value() == game_id.value())
            .max_by_key(|e| e.timestamp)
            .cloned()
    }
    
    fn find_by_player_id(&self, player_id: &PlayerId) -> Vec<GameHistoryEntry> {
        self.entries.values()
            .filter(|e| e.winner_ids.iter().any(|id| id.value() == player_id.value()))
            .cloned()
            .collect()
    }
    
    fn find_all(&self) -> Vec<GameHistoryEntry> {
        self.entries.values().cloned().collect()
    }
} 