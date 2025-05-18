use crate::domain::model::bet::BetAction;
use crate::domain::model::game::GameId;
use crate::domain::model::player::PlayerId;
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::service::game_rule::GameRuleService;

pub struct PlaceBetUseCase<R: GameRepository> {
    game_repository: R,
}

pub struct PlaceBetParams {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub action: BetAction,
    pub bet_amount: Option<u32>,
}

impl<R: GameRepository> PlaceBetUseCase<R> {
    pub fn new(game_repository: R) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&mut self, params: PlaceBetParams) -> Result<(), String> {
        // ゲームを取得
        let mut game = self.game_repository
            .find_by_id(&params.game_id)
            .ok_or_else(|| format!("ゲーム {} が見つかりません", params.game_id.value()))?;
        
        // プレイヤーのインデックスを取得
        let player_index = game.players().iter()
            .position(|p| p.id() == &params.player_id)
            .ok_or_else(|| format!("プレイヤー {} がこのゲームに参加していません", params.player_id))?;
        
        // プレイヤーのアクションを処理
        GameRuleService::process_action(
            &mut game,
            player_index,
            params.action,
            params.bet_amount
        ).map_err(|e| e.to_string())?;
        
        // 更新されたゲームを保存
        self.game_repository.save(&game)?;
        
        Ok(())
    }
} 