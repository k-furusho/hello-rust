#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::game::{Game, GameVariant};
    use crate::domain::model::player::Player;

    fn create_test_game() -> Game {
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        game.add_player(Player::new("テストプレイヤー".to_string(), 1000)).unwrap();
        game
    }

    #[test]
    fn ゲーム保存と取得() {
        let mut repo = InMemoryGameRepository::new();
        let game = create_test_game();
        let game_id = game.id().clone();
        
        // ゲームを保存
        let save_result = repo.save(&game);
        assert!(save_result.is_ok());
        
        // IDで検索
        let found_game = repo.find_by_id(&game_id);
        assert!(found_game.is_some());
        
        let found_game = found_game.unwrap();
        assert_eq!(found_game.id().value(), game.id().value());
        assert_eq!(found_game.variant(), game.variant());
        assert_eq!(found_game.players().len(), game.players().len());
    }

    #[test]
    fn 全ゲーム取得() {
        let mut repo = InMemoryGameRepository::new();
        
        // 最初は空
        let games = repo.find_all();
        assert_eq!(games.len(), 0);
        
        // 3つのゲームを追加
        let game1 = create_test_game();
        let game2 = create_test_game();
        let game3 = create_test_game();
        
        repo.save(&game1).unwrap();
        repo.save(&game2).unwrap();
        repo.save(&game3).unwrap();
        
        // 全ゲーム取得
        let games = repo.find_all();
        assert_eq!(games.len(), 3);
    }

    #[test]
    fn ゲーム削除() {
        let mut repo = InMemoryGameRepository::new();
        let game = create_test_game();
        let game_id = game.id().clone();
        
        // ゲームを保存
        repo.save(&game).unwrap();
        
        // 削除前に存在確認
        assert!(repo.find_by_id(&game_id).is_some());
        
        // ゲームを削除
        let delete_result = repo.delete(&game_id);
        assert!(delete_result.is_ok());
        
        // 削除後は取得できない
        assert!(repo.find_by_id(&game_id).is_none());
    }

    #[test]
    fn ゲーム更新() {
        let mut repo = InMemoryGameRepository::new();
        let mut game = create_test_game();
        let game_id = game.id().clone();
        
        // 最初のゲームを保存
        repo.save(&game).unwrap();
        
        // ゲームを変更（プレイヤー追加）
        game.add_player(Player::new("2人目".to_string(), 500)).unwrap();
        
        // 更新
        repo.save(&game).unwrap();
        
        // 更新されたゲームを取得
        let updated_game = repo.find_by_id(&game_id).unwrap();
        assert_eq!(updated_game.players().len(), 2);
        assert_eq!(updated_game.players()[1].name(), "2人目");
    }

    #[test]
    fn 存在しないゲーム検索() {
        let repo = InMemoryGameRepository::new();
        let non_existent_id = GameId::new();
        
        // 存在しないIDでの検索
        let result = repo.find_by_id(&non_existent_id);
        assert!(result.is_none());
    }

    #[test]
    fn スレッドセーフな操作() {
        use std::thread;
        use std::sync::Arc;
        
        let repo = Arc::new(std::sync::Mutex::new(InMemoryGameRepository::new()));
        let mut handles = vec![];
        
        // 複数スレッドから同時にゲームを追加
        for _ in 0..5 {
            let repo_clone = Arc::clone(&repo);
            let handle = thread::spawn(move || {
                let mut locked_repo = repo_clone.lock().unwrap();
                let game = create_test_game();
                locked_repo.save(&game).unwrap();
            });
            handles.push(handle);
        }
        
        // すべてのスレッドが完了するのを待つ
        for handle in handles {
            handle.join().unwrap();
        }
        
        // 全ゲーム取得して数を確認
        let games = repo.lock().unwrap().find_all();
        assert_eq!(games.len(), 5);
    }
} 