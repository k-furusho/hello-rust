#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn ゲーム作成イベント() {
        let game_id = GameId::new();
        let time = Utc::now();
        let event = GameEvent::GameCreated {
            game_id: game_id.clone(),
            variant: GameVariant::FiveCardDraw,
            small_blind: 5,
            big_blind: 10,
            time,
        };

        assert_eq!(event.event_type(), "GameCreated");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn ゲーム開始イベント() {
        let game_id = GameId::new();
        let time = Utc::now();
        let event = GameEvent::GameStarted {
            game_id: game_id.clone(),
            player_count: 4,
            time,
        };

        assert_eq!(event.event_type(), "GameStarted");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn プレイヤー追加イベント() {
        let game_id = GameId::new();
        let player_id = PlayerId::new();
        let time = Utc::now();
        let event = GameEvent::PlayerAdded {
            game_id: game_id.clone(),
            player_id: player_id.clone(),
            player_name: "テストプレイヤー".to_string(),
            initial_chips: 500,
            time,
        };

        assert_eq!(event.event_type(), "PlayerAdded");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn カード配布イベント() {
        let game_id = GameId::new();
        let player_id = PlayerId::new();
        let time = Utc::now();
        let event = GameEvent::CardsDealt {
            game_id: game_id.clone(),
            player_id: player_id.clone(),
            time,
        };

        assert_eq!(event.event_type(), "CardsDealt");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn ベッティングラウンド開始イベント() {
        let game_id = GameId::new();
        let time = Utc::now();
        let event = GameEvent::BettingRoundStarted {
            game_id: game_id.clone(),
            round: BettingRound::PreFlop,
            time,
        };

        assert_eq!(event.event_type(), "BettingRoundStarted");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn プレイヤーアクションイベント() {
        let game_id = GameId::new();
        let player_id = PlayerId::new();
        let time = Utc::now();
        let event = GameEvent::PlayerAction {
            game_id: game_id.clone(),
            player_id: player_id.clone(),
            action: BetAction::Raise,
            amount: Some(50),
            time,
        };

        assert_eq!(event.event_type(), "PlayerAction");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn カード交換イベント() {
        let game_id = GameId::new();
        let player_id = PlayerId::new();
        let time = Utc::now();
        let event = GameEvent::CardsExchanged {
            game_id: game_id.clone(),
            player_id: player_id.clone(),
            count: 3,
            time,
        };

        assert_eq!(event.event_type(), "CardsExchanged");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn コミュニティカード配布イベント() {
        let game_id = GameId::new();
        let time = Utc::now();
        let cards = vec![
            Card::new(Suit::Heart, 1).unwrap(),
            Card::new(Suit::Spade, 10).unwrap(),
            Card::new(Suit::Diamond, 5).unwrap(),
        ];
        let event = GameEvent::CommunityCardsDealt {
            game_id: game_id.clone(),
            cards: cards.clone(),
            time,
        };

        assert_eq!(event.event_type(), "CommunityCardsDealt");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }

    #[test]
    fn ゲーム終了イベント() {
        let game_id = GameId::new();
        let time = Utc::now();
        let winner_ids = vec![PlayerId::new(), PlayerId::new()];
        let event = GameEvent::GameEnded {
            game_id: game_id.clone(),
            winner_ids: winner_ids.clone(),
            pot_amount: 1000,
            time,
        };

        assert_eq!(event.event_type(), "GameEnded");
        assert_eq!(event.occurred_at(), time);
        assert_eq!(event.aggregate_id(), game_id.value());
    }
} 