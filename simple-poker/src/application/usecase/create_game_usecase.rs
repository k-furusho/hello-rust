use chrono::Utc;
use crate::domain::model::game::{Game, GameId, GameVariant};
use crate::domain::model::error::DomainError;
use crate::domain::model::event::{GameEvent, EventPublisher, EventSubscriber};
use crate::domain::repository::game_repository::GameRepository;

pub struct CreateGameUseCase<R: GameRepository, E: EventPublisher> {
    game_repository: R,
    event_publisher: E,
}

pub struct CreateGameParams {
    pub variant: GameVariant,
    pub small_blind: u32,
    pub big_blind: u32,
}

impl<R: GameRepository, E: EventPublisher> CreateGameUseCase<R, E> {
    pub fn new(game_repository: R, event_publisher: E) -> Self {
        Self { game_repository, event_publisher }
    }
    
    pub fn execute(&mut self, params: CreateGameParams) -> Result<GameId, DomainError> {
        let game = Game::new(params.variant, params.small_blind, params.big_blind)
            .map_err(|e| DomainError::InvalidGameOperation(e.to_string()))?;
        
        let game_id = game.id().clone();
        self.game_repository.save(&game)?;
        
        let event = GameEvent::GameCreated {
            game_id: game_id.clone(),
            variant: params.variant,
            small_blind: params.small_blind,
            big_blind: params.big_blind,
            time: Utc::now(),
        };
        
        self.event_publisher.publish(event)?;
        
        Ok(game_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repository::inmemory::game_repository_inmemory::InMemoryGameRepository;
    use crate::infrastructure::service::event::inmemory_event_publisher::InMemoryEventPublisher;
    use std::sync::{Arc, Mutex};
    
    #[test]
    fn ゲーム作成成功() {
        let game_repo = InMemoryGameRepository::new();
        let event_publisher = InMemoryEventPublisher::new();
        
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        let mut event_publisher_with_subscriber = event_publisher.clone();
        event_publisher_with_subscriber.subscribe(Box::new(move |_| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        }));
        
        let mut usecase = CreateGameUseCase::new(game_repo.clone(), event_publisher_with_subscriber);
        
        let params = CreateGameParams {
            variant: GameVariant::FiveCardDraw,
            small_blind: 5,
            big_blind: 10,
        };
        
        let result = usecase.execute(params);
        
        assert!(result.is_ok(), "ゲーム作成に失敗しました");
        let game_id = result.unwrap();
        
        let saved_game = game_repo.find_by_id(&game_id);
        assert!(saved_game.is_some(), "ゲームがリポジトリに保存されていません");
        
        assert_eq!(*counter.lock().unwrap(), 1, "イベントが発行されていません");
    }
    
    #[test]
    fn 無効なブラインド値でのエラー() {
        let game_repo = InMemoryGameRepository::new();
        let event_publisher = InMemoryEventPublisher::new();
        let mut usecase = CreateGameUseCase::new(game_repo, event_publisher);
        
        let params = CreateGameParams {
            variant: GameVariant::FiveCardDraw,
            small_blind: 20,
            big_blind: 10,
        };
        
        let result = usecase.execute(params);
        
        assert!(result.is_err(), "不正なブラインド値でもエラーになりません");
        match result {
            Err(DomainError::InvalidGameOperation(_)) => {
            },
            _ => panic!("期待したエラー型ではありません"),
        }
    }
} 