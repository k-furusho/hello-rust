use crate::domain::model::game::GameId;
use crate::domain::model::player::PlayerId;
use crate::domain::repository::game_repository::GameRepository;

pub struct ExchangeCardsUseCase<R: GameRepository> {
    game_repository: R,
}

pub struct ExchangeCardsParams {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_indices: Vec<usize>,
}

impl<R: GameRepository> ExchangeCardsUseCase<R> {
    pub fn new(game_repository: R) -> Self {
        Self { game_repository }
    }
    
    pub fn execute(&mut self, params: ExchangeCardsParams) -> Result<(), String> {
        // ゲームを取得
        let mut game = self.game_repository
            .find_by_id(&params.game_id)
            .ok_or_else(|| format!("ゲーム {} が見つかりません", params.game_id.value()))?;
        
        // プレイヤーのインデックスを取得
        let player_index = game.players().iter()
            .position(|p| p.id() == &params.player_id)
            .ok_or_else(|| format!("プレイヤー {} がこのゲームに参加していません", params.player_id))?;
        
        // カード交換を実行
        game.exchange_cards(player_index, &params.card_indices)
            .map_err(|e| e.to_string())?;
        
        // 更新されたゲームを保存
        self.game_repository.save(&game)?;
        
        Ok(())
    }
} 