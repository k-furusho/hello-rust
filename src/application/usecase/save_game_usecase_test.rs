#[cfg(test)]
mod tests {
    use chrono::Utc;
    
    use crate::application::usecase::save_game_usecase::{SaveGameParams, SaveGameUseCase};
    use crate::domain::model::game::{Game, GameId, GameVariant};
    use crate::domain::model::player::{Player, PlayerId};
    use crate::domain::repository::game_history_repository::{GameHistoryEntry, GameHistoryRepository};
    use crate::domain::repository::game_repository::GameRepository;
    use crate::domain::error::{RepositoryError, RepositoryResult};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    
    // テスト用のモックリポジトリ
    struct MockGameRepository {
        games: HashMap<String, Game>,
    }
    
    impl MockGameRepository {
        fn new() -> Self {
            Self { games: HashMap::new() }
        }
    }
    
    impl GameRepository for MockGameRepository {
        fn save(&mut self, game: &Game) -> RepositoryResult<()> {
            self.games.insert(game.id().value().to_string(), game.clone());
            Ok(())
        }
        
        fn find_by_id(&self, id: &GameId) -> Option<Game> {
            self.games.get(id.value()).cloned()
        }
        
        fn find_all(&self) -> Vec<Game> {
            self.games.values().cloned().collect()
        }
        
        fn delete(&mut self, id: &GameId) -> RepositoryResult<()> {
            self.games.remove(id.value());
            Ok(())
        }
    }
    
    struct MockGameHistoryRepository {
        entries: Vec<GameHistoryEntry>,
    }
    
    impl MockGameHistoryRepository {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
    }
    
    impl GameHistoryRepository for MockGameHistoryRepository {
        fn save(&mut self, entry: &GameHistoryEntry) -> RepositoryResult<()> {
            self.entries.push(entry.clone());
            Ok(())
        }
        
        fn find_by_game_id(&self, game_id: &GameId) -> Option<GameHistoryEntry> {
            self.entries.iter()
                .filter(|e| e.game_id.value() == game_id.value())
                .max_by_key(|e| e.timestamp)
                .cloned()
        }
        
        fn find_by_player_id(&self, player_id: &PlayerId) -> Vec<GameHistoryEntry> {
            self.entries.iter()
                .filter(|e| e.winner_ids.iter().any(|id| id.value() == player_id.value()))
                .cloned()
                .collect()
        }
        
        fn find_all(&self) -> Vec<GameHistoryEntry> {
            self.entries.clone()
        }
    }
    
    #[test]
    fn test_save_game_success() {
        // テスト用のゲームとプレイヤーを作成
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).expect("ゲーム作成失敗");
        let player1 = Player::new("プレイヤー1".to_string(), 100);
        let player2 = Player::new("プレイヤー2".to_string(), 100);
        
        game.add_player(player1.clone()).expect("プレイヤー追加失敗");
        game.add_player(player2).expect("プレイヤー追加失敗");
        
        // モックリポジトリを作成
        let mut mock_game_repo = MockGameRepository::new();
        mock_game_repo.save(&game).expect("ゲーム保存失敗");
        
        let mut mock_history_repo = MockGameHistoryRepository::new();
        
        // ユースケースを作成
        let mut usecase = SaveGameUseCase::new(mock_game_repo, mock_history_repo);
        
        // パラメータを作成
        let params = SaveGameParams {
            game_id: game.id().clone(),
            winner_ids: vec![player1.id().clone()],
        };
        
        // 実行
        let result = usecase.execute(params);
        
        // 検証
        assert!(result.is_ok());
    }
} 