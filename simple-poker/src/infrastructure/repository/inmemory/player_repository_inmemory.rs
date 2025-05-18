use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::model::player::{Player, PlayerId};
use crate::domain::repository::player_repository::PlayerRepository;

#[derive(Clone)]
pub struct InMemoryPlayerRepository {
    players: Arc<Mutex<HashMap<String, Player>>>,
}

impl InMemoryPlayerRepository {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryPlayerRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerRepository for InMemoryPlayerRepository {
    fn save(&mut self, player: &Player) -> Result<(), String> {
        let mut players = self.players.lock().map_err(|_| "ロックの取得に失敗しました".to_string())?;
        players.insert(player.id().value().to_string(), player.clone());
        Ok(())
    }
    
    fn find_by_id(&self, id: &PlayerId) -> Option<Player> {
        let players = self.players.lock().ok()?;
        players.get(id.value()).cloned()
    }
    
    fn find_all(&self) -> Vec<Player> {
        let players = match self.players.lock() {
            Ok(players) => players,
            Err(_) => return Vec::new(),
        };
        players.values().cloned().collect()
    }
    
    fn delete(&mut self, id: &PlayerId) -> Result<(), String> {
        let mut players = self.players.lock().map_err(|_| "ロックの取得に失敗しました".to_string())?;
        players.remove(id.value());
        Ok(())
    }
} 