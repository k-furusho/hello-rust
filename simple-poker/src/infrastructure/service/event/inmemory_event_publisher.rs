use std::sync::{Arc, Mutex};
use crate::domain::model::event::{GameEvent, EventPublisher, EventSubscriber};
use crate::domain::model::error::DomainError;

// 型エイリアスを導入して複雑な型を単純化
type EventCallback = Box<dyn Fn(&GameEvent) + Send + 'static>;
type Subscribers = Arc<Mutex<Vec<EventCallback>>>;

#[derive(Clone)]
pub struct InMemoryEventPublisher {
    subscribers: Subscribers,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for InMemoryEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventPublisher for InMemoryEventPublisher {
    fn publish(&self, event: GameEvent) -> Result<(), DomainError> {
        // イベントをすべてのサブスクライバーに通知
        if let Ok(subscribers) = self.subscribers.lock() {
            for subscriber in subscribers.iter() {
                subscriber(&event);
            }
        } else {
            return Err(DomainError::InvalidState("イベントパブリッシャーのロック取得に失敗しました".into()));
        }
        
        Ok(())
    }
}

impl EventSubscriber for InMemoryEventPublisher {
    fn subscribe(&mut self, callback: EventCallback) {
        if let Ok(mut subscribers) = self.subscribers.lock() {
            subscribers.push(callback);
        }
    }
}

// 簡易的な使用例
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::game::{GameId, GameVariant};
    use chrono::Utc;
    
    #[test]
    fn イベント発行と購読() {
        let mut publisher = InMemoryEventPublisher::new();
        
        // イベントを処理するためのカウンタ
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        
        // サブスクライバーを登録
        publisher.subscribe(Box::new(move |event| {
            match event {
                GameEvent::GameCreated { .. } => {
                    let mut count = counter_clone.lock().unwrap();
                    *count += 1;
                },
                _ => {}
            }
        }));
        
        // イベントを発行
        let event = GameEvent::GameCreated {
            game_id: GameId::new(),
            variant: GameVariant::FiveCardDraw,
            small_blind: 5,
            big_blind: 10,
            time: Utc::now(),
        };
        
        // イベントを発行
        publisher.publish(event).unwrap();
        
        // カウンタが増加したことを確認
        assert_eq!(*counter.lock().unwrap(), 1);
    }
} 