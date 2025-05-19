#[cfg(test)]
mod tests {
    use crate::domain::model::game::GameId;
    use crate::domain::model::player::PlayerId;
    use crate::domain::repository::game_history_repository::{GameHistoryEntry, GameHistoryRepository};
    use crate::infrastructure::repository::file::game_history_repository_file::FileGameHistoryRepository;
    use tempfile::TempDir;
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
    fn ファイルへのエントリー保存と読み込み() {
        // 一時ディレクトリを作成
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗");
        let temp_path = temp_dir.path();
        
        // リポジトリを初期化
        let mut repo = FileGameHistoryRepository::new(temp_path)
            .expect("リポジトリの初期化に失敗");
        
        let now = Utc::now();
        
        // テストエントリを作成
        let entry = create_test_entry(
            "game-test-1",
            vec!["player-a", "player-b"],
            1000,
            now,
            "FiveCardDraw",
            4,
        );
        
        // エントリを保存
        let result = repo.save(&entry);
        assert!(result.is_ok(), "エントリの保存に失敗");
        
        // リポジトリを再度開いて、正しく読み込まれるか確認
        let repo2 = FileGameHistoryRepository::new(temp_path)
            .expect("リポジトリの再オープンに失敗");
        
        // ゲームIDで検索
        let found = repo2.find_by_game_id(&entry.game_id);
        assert!(found.is_some(), "保存したエントリが見つかりません");
        
        let found_entry = found.unwrap();
        assert_eq!(found_entry.game_id.value(), entry.game_id.value());
        assert_eq!(found_entry.pot_amount, entry.pot_amount);
        assert_eq!(found_entry.variant, entry.variant);
        assert_eq!(found_entry.player_count, entry.player_count);
        assert_eq!(found_entry.winner_ids.len(), entry.winner_ids.len());
    }
    
    #[test]
    fn プレイヤー_idによる検索() {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗");
        let temp_path = temp_dir.path();
        
        let mut repo = FileGameHistoryRepository::new(temp_path)
            .expect("リポジトリの初期化に失敗");
        
        let now = Utc::now();
        
        // プレイヤー1が勝者のエントリー
        let entry1 = create_test_entry(
            "game-2",
            vec!["player-1"],
            500,
            now,
            "FiveCardDraw",
            3,
        );
        
        // プレイヤー2が勝者のエントリー
        let entry2 = create_test_entry(
            "game-3",
            vec!["player-2"],
            800,
            now + Duration::minutes(10),
            "FiveCardDraw",
            3,
        );
        
        // 両方のプレイヤーが勝者のエントリー
        let entry3 = create_test_entry(
            "game-4",
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
        assert_eq!(player1_entries.len(), 2); // game-2とgame-4
        
        // プレイヤー2で検索
        let player2_id = PlayerId::from_string("player-2".to_string());
        let player2_entries = repo.find_by_player_id(&player2_id);
        assert_eq!(player2_entries.len(), 2); // game-3とgame-4
        
        // 存在しないプレイヤーで検索
        let player3_id = PlayerId::from_string("player-3".to_string());
        let player3_entries = repo.find_by_player_id(&player3_id);
        assert_eq!(player3_entries.len(), 0);
    }
    
    #[test]
    fn 全エントリー取得() {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗");
        let temp_path = temp_dir.path();
        
        let mut repo = FileGameHistoryRepository::new(temp_path)
            .expect("リポジトリの初期化に失敗");
        
        let now = Utc::now();
        
        // 複数のエントリーを作成して保存
        let entries = vec![
            create_test_entry("game-a", vec!["player-1"], 500, now, "FiveCardDraw", 3),
            create_test_entry("game-b", vec!["player-2"], 800, now + Duration::minutes(5), "TexasHoldem", 4),
            create_test_entry("game-c", vec!["player-3"], 1200, now + Duration::minutes(10), "Omaha", 5),
        ];
        
        for entry in &entries {
            repo.save(entry).unwrap();
        }
        
        // 全てのエントリーを取得
        let all_entries = repo.find_all();
        assert_eq!(all_entries.len(), entries.len());
        
        // 各エントリーのゲームIDを検証
        let game_ids: Vec<String> = all_entries
            .iter()
            .map(|e| e.game_id.value().to_string())
            .collect();
            
        assert!(game_ids.contains(&"game-a".to_string()));
        assert!(game_ids.contains(&"game-b".to_string()));
        assert!(game_ids.contains(&"game-c".to_string()));
    }
    
    #[test]
    fn 同一ゲームの最新エントリー取得() {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗");
        let temp_path = temp_dir.path();
        
        let mut repo = FileGameHistoryRepository::new(temp_path)
            .expect("リポジトリの初期化に失敗");
        
        let now = Utc::now();
        
        // 同じゲームIDで異なるタイムスタンプのエントリーを作成
        let entry1 = create_test_entry(
            "game-duplicate",
            vec!["player-1"],
            500,
            now,
            "FiveCardDraw",
            3,
        );
        
        let entry2 = create_test_entry(
            "game-duplicate",
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
        let game_id = GameId::from_string("game-duplicate".to_string());
        let found = repo.find_by_game_id(&game_id).unwrap();
        
        // タイムスタンプが新しい方（entry2）が返されることを確認
        assert_eq!(found.pot_amount, 1000);
        assert_eq!(found.winner_ids.len(), 2);
    }
    
    #[test]
    fn ファイルの存在しないディレクトリでの初期化() {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗");
        let non_existent_dir = temp_dir.path().join("non_existent");
        
        // 存在しないディレクトリでリポジトリを初期化
        let result = FileGameHistoryRepository::new(&non_existent_dir);
        
        // ディレクトリが存在しなくても成功して、作成されるはず
        assert!(result.is_ok());
        assert!(non_existent_dir.exists());
    }
} 