#[cfg(test)]
mod tests {
    use crate::domain::model::game::GameId;
    use crate::domain::model::player::PlayerId;
    use crate::domain::repository::game_history_repository::{GameHistoryEntry, GameHistoryRepository};
    use crate::infrastructure::repository::inmemory::game_history_repository_inmemory::InMemoryGameHistoryRepository;
    use chrono::{DateTime, Duration, Utc};

    // テスト用のゲーム履歴エントリを作成
    fn create_test_entry(
        game_id: &str,
        winner_ids: Vec<&str>,
        pot_amount: u32,
        timestamp: DateTime<Utc>,
        variant: &str,
        player_count: usize,
    ) -> GameHistoryEntry {
        GameHistoryEntry {
            game_id: GameId::from_string(game_id.to_string()),
            timestamp,
            winner_ids: winner_ids.into_iter()
                .map(|id| PlayerId::from_string(id.to_string()))
                .collect(),
            pot_amount,
            variant: variant.to_string(),
            player_count,
        }
    }

    #[test]
    fn エントリー保存と取得() {
        let mut repo = InMemoryGameHistoryRepository::new();
        let now = Utc::now();
        
        // エントリーを作成
        let entry = create_test_entry(
            "game-1",
            vec!["player-1", "player-2"],
            1000,
            now,
            "FiveCardDraw",
            5,
        );
        
        // 保存
        let result = repo.save(&entry);
        assert!(result.is_ok());
        
        // ゲームIDで検索
        let found = repo.find_by_game_id(&entry.game_id);
        assert!(found.is_some());
        
        let found_entry = found.unwrap();
        assert_eq!(found_entry.game_id.value(), entry.game_id.value());
        assert_eq!(found_entry.pot_amount, 1000);
        assert_eq!(found_entry.variant, "FiveCardDraw");
        assert_eq!(found_entry.player_count, 5);
        assert_eq!(found_entry.winner_ids.len(), 2);
    }
    
    #[test]
    fn プレイヤーIDによる検索() {
        let mut repo = InMemoryGameHistoryRepository::new();
        let now = Utc::now();
        
        // プレイヤー1が勝者のエントリー
        let entry1 = create_test_entry(
            "game-1",
            vec!["player-1"],
            500,
            now,
            "FiveCardDraw",
            3,
        );
        
        // プレイヤー2が勝者のエントリー
        let entry2 = create_test_entry(
            "game-2",
            vec!["player-2"],
            800,
            now + Duration::minutes(10),
            "FiveCardDraw",
            3,
        );
        
        // 両方のプレイヤーが勝者のエントリー
        let entry3 = create_test_entry(
            "game-3",
            vec!["player-1", "player-2"],
            1200,
            now + Duration::minutes(20),
            "FiveCardDraw",
            4,
        );
        
        // 保存
        repo.save(&entry1).unwrap();
        repo.save(&entry2).unwrap();
        repo.save(&entry3).unwrap();
        
        // プレイヤー1で検索
        let player1_id = PlayerId::from_string("player-1".to_string());
        let player1_entries = repo.find_by_player_id(&player1_id);
        assert_eq!(player1_entries.len(), 2); // game-1とgame-3
        
        // プレイヤー2で検索
        let player2_id = PlayerId::from_string("player-2".to_string());
        let player2_entries = repo.find_by_player_id(&player2_id);
        assert_eq!(player2_entries.len(), 2); // game-2とgame-3
        
        // 存在しないプレイヤーで検索
        let player3_id = PlayerId::from_string("player-3".to_string());
        let player3_entries = repo.find_by_player_id(&player3_id);
        assert_eq!(player3_entries.len(), 0);
    }
    
    #[test]
    fn 全エントリー取得() {
        let mut repo = InMemoryGameHistoryRepository::new();
        let now = Utc::now();
        
        // 複数のエントリーを作成して保存
        let entry1 = create_test_entry(
            "game-1",
            vec!["player-1"],
            500,
            now,
            "FiveCardDraw",
            3,
        );
        
        let entry2 = create_test_entry(
            "game-2",
            vec!["player-2"],
            800,
            now + Duration::minutes(10),
            "FiveCardDraw",
            3,
        );
        
        repo.save(&entry1).unwrap();
        repo.save(&entry2).unwrap();
        
        // 全てのエントリーを取得
        let all_entries = repo.find_all();
        assert_eq!(all_entries.len(), 2);
    }
    
    #[test]
    fn 同一ゲームの最新エントリー取得() {
        let mut repo = InMemoryGameHistoryRepository::new();
        let now = Utc::now();
        
        // 同じゲームIDで異なるタイムスタンプのエントリーを作成
        let entry1 = create_test_entry(
            "game-1",
            vec!["player-1"],
            500,
            now,
            "FiveCardDraw",
            3,
        );
        
        let entry2 = create_test_entry(
            "game-1",
            vec!["player-1", "player-2"],
            1000,
            now + Duration::minutes(30),
            "FiveCardDraw",
            3,
        );
        
        // 保存（順序は関係ない）
        repo.save(&entry2).unwrap();
        repo.save(&entry1).unwrap();
        
        // ゲームIDで検索すると最新のエントリーが返される
        let game_id = GameId::from_string("game-1".to_string());
        let found = repo.find_by_game_id(&game_id).unwrap();
        
        // タイムスタンプが新しい方（entry2）が返されることを確認
        assert_eq!(found.pot_amount, 1000);
        assert_eq!(found.winner_ids.len(), 2);
    }
} 