#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::player::Player;

    #[test]
    fn プレイヤー保存と取得() {
        let mut repo = InMemoryPlayerRepository::new();
        let player = Player::new("テストプレイヤー".to_string(), 1000);
        let player_id = player.id().clone();
        
        // プレイヤーを保存
        let save_result = repo.save(&player);
        assert!(save_result.is_ok());
        
        // IDで検索
        let found_player = repo.find_by_id(&player_id);
        assert!(found_player.is_some());
        
        let found_player = found_player.unwrap();
        assert_eq!(found_player.id().value(), player.id().value());
        assert_eq!(found_player.name(), player.name());
        assert_eq!(found_player.chips(), player.chips());
    }

    #[test]
    fn 全プレイヤー取得() {
        let mut repo = InMemoryPlayerRepository::new();
        
        // 最初は空
        let players = repo.find_all();
        assert_eq!(players.len(), 0);
        
        // 3つのプレイヤーを追加
        let player1 = Player::new("プレイヤー1".to_string(), 100);
        let player2 = Player::new("プレイヤー2".to_string(), 200);
        let player3 = Player::new("プレイヤー3".to_string(), 300);
        
        repo.save(&player1).unwrap();
        repo.save(&player2).unwrap();
        repo.save(&player3).unwrap();
        
        // 全プレイヤー取得
        let players = repo.find_all();
        assert_eq!(players.len(), 3);
        
        // プレイヤー名の確認
        let names: Vec<&str> = players.iter().map(|p| p.name()).collect();
        assert!(names.contains(&"プレイヤー1"));
        assert!(names.contains(&"プレイヤー2"));
        assert!(names.contains(&"プレイヤー3"));
    }

    #[test]
    fn プレイヤー削除() {
        let mut repo = InMemoryPlayerRepository::new();
        let player = Player::new("削除対象".to_string(), 500);
        let player_id = player.id().clone();
        
        // プレイヤーを保存
        repo.save(&player).unwrap();
        
        // 削除前に存在確認
        assert!(repo.find_by_id(&player_id).is_some());
        
        // プレイヤーを削除
        let delete_result = repo.delete(&player_id);
        assert!(delete_result.is_ok());
        
        // 削除後は取得できない
        assert!(repo.find_by_id(&player_id).is_none());
    }

    #[test]
    fn プレイヤー更新() {
        let mut repo = InMemoryPlayerRepository::new();
        
        // プレイヤー作成と保存
        let mut player = Player::new("名前変更前".to_string(), 100);
        let player_id = player.id().clone();
        repo.save(&player).unwrap();
        
        // プレイヤー情報を変更（チップを増やす）
        player.add_chips(900); // 100 -> 1000
        
        // 更新
        repo.save(&player).unwrap();
        
        // 更新されたプレイヤーを取得
        let updated_player = repo.find_by_id(&player_id).unwrap();
        assert_eq!(updated_player.chips(), 1000);
    }

    #[test]
    fn 存在しないプレイヤー検索() {
        let repo = InMemoryPlayerRepository::new();
        let non_existent_id = PlayerId::new();
        
        // 存在しないIDでの検索
        let result = repo.find_by_id(&non_existent_id);
        assert!(result.is_none());
    }

    #[test]
    fn スレッドセーフな操作() {
        use std::thread;
        use std::sync::Arc;
        
        let repo = Arc::new(std::sync::Mutex::new(InMemoryPlayerRepository::new()));
        let mut handles = vec![];
        
        // 複数スレッドから同時にプレイヤーを追加
        for i in 0..5 {
            let repo_clone = Arc::clone(&repo);
            let handle = thread::spawn(move || {
                let mut locked_repo = repo_clone.lock().unwrap();
                let player = Player::new(format!("プレイヤー{}", i), 100 * (i + 1));
                locked_repo.save(&player).unwrap();
            });
            handles.push(handle);
        }
        
        // すべてのスレッドが完了するのを待つ
        for handle in handles {
            handle.join().unwrap();
        }
        
        // 全プレイヤー取得して数を確認
        let players = repo.lock().unwrap().find_all();
        assert_eq!(players.len(), 5);
    }
} 