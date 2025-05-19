use crate::domain::model::game::GameId;
use crate::domain::model::player::{Player, PlayerId};
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::repository::player_repository::PlayerRepository;

pub struct AddPlayerUseCase<G: GameRepository, P: PlayerRepository> {
    game_repository: G,
    player_repository: P,
}

pub struct AddPlayerParams {
    pub game_id: GameId,
    pub player_name: String,
    pub initial_chips: u32,
}

impl<G: GameRepository, P: PlayerRepository> AddPlayerUseCase<G, P> {
    pub fn new(game_repository: G, player_repository: P) -> Self {
        Self {
            game_repository,
            player_repository,
        }
    }
    
    pub fn execute(&mut self, params: AddPlayerParams) -> Result<PlayerId, String> {
        // ゲームを取得
        let mut game = self.game_repository
            .find_by_id(&params.game_id)
            .ok_or_else(|| format!("ゲーム {} が見つかりません", params.game_id.value()))?;
        
        // プレイヤーを作成
        let player = Player::new(params.player_name, params.initial_chips);
        let player_id = player.id().clone();
        
        // ゲームにプレイヤーを追加
        game.add_player(player.clone())
            .map_err(|e| e.to_string())?;
        
        // プレイヤーを保存
        self.player_repository.save(&player)?;
        
        // 更新されたゲームを保存
        self.game_repository.save(&game)?;
        
        Ok(player_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::game::{Game, GameVariant};
    use crate::infrastructure::repository::inmemory::game_repository_inmemory::InMemoryGameRepository;
    use crate::infrastructure::repository::inmemory::player_repository_inmemory::InMemoryPlayerRepository;
    
    #[test]
    fn プレイヤー追加_正常系() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        let player_repo = InMemoryPlayerRepository::new();
        
        // ゲームを作成
        let game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = AddPlayerUseCase::new(game_repo.clone(), player_repo.clone());
        
        let params = AddPlayerParams {
            game_id: game_id.clone(),
            player_name: "テストプレイヤー".to_string(),
            initial_chips: 1000,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_ok());
        
        let player_id = result.unwrap();
        
        // ゲームの状態を確認
        let updated_game = game_repo.find_by_id(&game_id).unwrap();
        assert_eq!(updated_game.players().len(), 1);
        assert_eq!(updated_game.players()[0].name(), "テストプレイヤー");
        assert_eq!(updated_game.players()[0].chips(), 1000);
        
        // プレイヤーリポジトリの状態を確認
        let saved_player = player_repo.find_by_id(&player_id);
        assert!(saved_player.is_some());
        let saved_player = saved_player.unwrap();
        assert_eq!(saved_player.name(), "テストプレイヤー");
        assert_eq!(saved_player.chips(), 1000);
    }
    
    #[test]
    fn 存在しないゲームへのプレイヤー追加() {
        let game_repo = InMemoryGameRepository::new();
        let player_repo = InMemoryPlayerRepository::new();
        
        let mut usecase = AddPlayerUseCase::new(game_repo, player_repo);
        
        let params = AddPlayerParams {
            game_id: GameId::new(), // 存在しないゲームID
            player_name: "テストプレイヤー".to_string(),
            initial_chips: 1000,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("見つかりません"));
    }
    
    #[test]
    fn 既存ゲームに複数プレイヤー追加() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        let player_repo = InMemoryPlayerRepository::new();
        
        // ゲームを作成
        let game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = AddPlayerUseCase::new(game_repo.clone(), player_repo.clone());
        
        // 1人目のプレイヤーを追加
        let params1 = AddPlayerParams {
            game_id: game_id.clone(),
            player_name: "プレイヤー1".to_string(),
            initial_chips: 500,
        };
        let result1 = usecase.execute(params1);
        assert!(result1.is_ok());
        
        // 2人目のプレイヤーを追加
        let params2 = AddPlayerParams {
            game_id: game_id.clone(),
            player_name: "プレイヤー2".to_string(),
            initial_chips: 1000,
        };
        let result2 = usecase.execute(params2);
        assert!(result2.is_ok());
        
        // ゲームの状態を確認
        let updated_game = game_repo.find_by_id(&game_id).unwrap();
        assert_eq!(updated_game.players().len(), 2);
        assert_eq!(updated_game.players()[0].name(), "プレイヤー1");
        assert_eq!(updated_game.players()[1].name(), "プレイヤー2");
        
        // プレイヤーリポジトリの状態を確認
        let all_players = player_repo.find_all();
        assert_eq!(all_players.len(), 2);
    }
    
    #[test]
    fn 開始済みゲームへのプレイヤー追加失敗() {
        // 準備
        let mut game_repo = InMemoryGameRepository::new();
        let player_repo = InMemoryPlayerRepository::new();
        
        // ゲームを作成して開始する
        let mut game = Game::new(GameVariant::FiveCardDraw, 5, 10).unwrap();
        // ゲームを開始するには最低2人のプレイヤーが必要
        game.add_player(Player::new("既存プレイヤー1".to_string(), 1000)).unwrap();
        game.add_player(Player::new("既存プレイヤー2".to_string(), 1000)).unwrap();
        game.start_game().unwrap(); // ゲームを開始
        
        let game_id = game.id().clone();
        
        // ゲームをリポジトリに保存
        game_repo.save(&game).unwrap();
        
        // ユースケース実行
        let mut usecase = AddPlayerUseCase::new(game_repo, player_repo);
        
        let params = AddPlayerParams {
            game_id,
            player_name: "新規プレイヤー".to_string(),
            initial_chips: 1000,
        };
        
        let result = usecase.execute(params);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("開始"));
    }
} 