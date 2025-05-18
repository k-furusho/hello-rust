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