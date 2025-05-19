#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::domain::model::event::{GameEvent, EventPublisher, EventSubscriber};
    use crate::domain::model::game::{GameId, GameVariant};
    use crate::domain::model::player::PlayerId;
    use crate::domain::model::bet::BetAction;
    use crate::infrastructure::service::event::inmemory_event_publisher::InMemoryEventPublisher;
    use chrono::Utc;

    #[test]
    fn イベント発行と購読_ゲーム作成() {
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
        
        publisher.publish(event).unwrap();
        
        // カウンタが増加したことを確認
        assert_eq!(*counter.lock().unwrap(), 1);
    }

    #[test]
    fn 複数のサブスクライバー_異なるイベント() {
        let mut publisher = InMemoryEventPublisher::new();
        
        let game_id = GameId::new();
        let player_id = PlayerId::from_string("player-1".to_string());
        
        // ゲーム作成イベントのカウンター
        let game_created_counter = Arc::new(Mutex::new(0));
        let game_created_counter_clone = game_created_counter.clone();
        
        // プレイヤー行動イベントのカウンター
        let player_action_counter = Arc::new(Mutex::new(0));
        let player_action_counter_clone = player_action_counter.clone();
        
        // ゲーム作成イベントを監視するサブスクライバー
        publisher.subscribe(Box::new(move |event| {
            if let GameEvent::GameCreated { .. } = event {
                let mut count = game_created_counter_clone.lock().unwrap();
                *count += 1;
            }
        }));
        
        // プレイヤー行動イベントを監視するサブスクライバー
        publisher.subscribe(Box::new(move |event| {
            if let GameEvent::PlayerAction { .. } = event {
                let mut count = player_action_counter_clone.lock().unwrap();
                *count += 1;
            }
        }));
        
        // 各種イベントを発行
        let game_created_event = GameEvent::GameCreated {
            game_id: game_id.clone(),
            variant: GameVariant::FiveCardDraw,
            small_blind: 5,
            big_blind: 10,
            time: Utc::now(),
        };
        
        let player_action_event = GameEvent::PlayerAction {
            game_id,
            player_id,
            action: BetAction::Call,
            amount: Some(10),
            time: Utc::now(),
        };
        
        publisher.publish(game_created_event).unwrap();
        publisher.publish(player_action_event).unwrap();
        
        // 各カウンターを確認
        assert_eq!(*game_created_counter.lock().unwrap(), 1);
        assert_eq!(*player_action_counter.lock().unwrap(), 1);
    }
    
    #[test]
    fn 同一イベント複数回発行() {
        let mut publisher = InMemoryEventPublisher::new();
        
        let game_id = GameId::new();
        
        // イベントカウンター
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        
        // サブスクライバー登録
        publisher.subscribe(Box::new(move |_| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        }));
        
        // 同じイベントを3回発行
        let event = GameEvent::GameStarted {
            game_id,
            player_count: 4,
            time: Utc::now(),
        };
        
        publisher.publish(event.clone()).unwrap();
        publisher.publish(event.clone()).unwrap();
        publisher.publish(event).unwrap();
        
        // 3回カウントされていることを確認
        assert_eq!(*counter.lock().unwrap(), 3);
    }
    
    #[test]
    fn エラー発生時のテスト() {
        // ロック取得失敗をシミュレートするのは難しいため、
        // 通常の動作確認のみを行う
        let publisher = InMemoryEventPublisher::new();
        
        let event = GameEvent::GameEnded {
            game_id: GameId::new(),
            winner_ids: vec![PlayerId::new()],
            pot_amount: 1000,
            time: Utc::now(),
        };
        
        let result = publisher.publish(event);
        assert!(result.is_ok());
    }
    
    #[test]
    fn デフォルト実装の確認() {
        let publisher = InMemoryEventPublisher::default();
        
        let event = GameEvent::GameEnded {
            game_id: GameId::new(),
            winner_ids: vec![PlayerId::new()],
            pot_amount: 1000,
            time: Utc::now(),
        };
        
        let result = publisher.publish(event);
        assert!(result.is_ok());
    }
} 