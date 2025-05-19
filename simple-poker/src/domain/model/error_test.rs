#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn エラー作成と表示() {
        let card_error = DomainError::InvalidCard("エース以外のカードです".to_string());
        let game_error = DomainError::InvalidGameOperation("ゲームが開始されていません".to_string());
        let player_error = DomainError::InvalidPlayerOperation("プレイヤーは既にフォールドしています".to_string());
        let bet_error = DomainError::InvalidBet("チップが足りません".to_string());
        let not_found = DomainError::ResourceNotFound("ゲームが見つかりません".to_string());
        let state_error = DomainError::InvalidState("不正な状態です".to_string());

        // エラーメッセージの確認
        assert!(card_error.to_string().contains("無効なカード"));
        assert!(game_error.to_string().contains("無効なゲーム操作"));
        assert!(player_error.to_string().contains("無効なプレイヤー操作"));
        assert!(bet_error.to_string().contains("無効なベット"));
        assert!(not_found.to_string().contains("リソースが見つかりません"));
        assert!(state_error.to_string().contains("不正な状態"));
    }

    #[test]
    fn 静的文字列からの変換() {
        let error: DomainError = "テストエラー".into();
        match error {
            DomainError::InvalidState(msg) => assert_eq!(msg, "テストエラー".to_string()),
            _ => panic!("期待した型のエラーではありません"),
        }
    }

    #[test]
    fn 文字列からの変換() {
        let message = String::from("エラーメッセージ");
        let error: DomainError = message.clone().into();
        match error {
            DomainError::InvalidState(msg) => assert_eq!(msg, message),
            _ => panic!("期待した型のエラーではありません"),
        }
    }

    #[test]
    fn ドメインエラーから文字列への変換() {
        let error = DomainError::InvalidCard("不正なカード".to_string());
        let error_string: String = error.into();
        assert!(error_string.contains("無効なカード"));
    }

    #[test]
    fn ドメインエラー参照から文字列への変換() {
        let error = DomainError::InvalidBet("ベットエラー".to_string());
        let error_string: String = (&error).into();
        assert!(error_string.contains("無効なベット"));
    }

    #[test]
    fn as_ref実装の確認() {
        let error = DomainError::InvalidCard("不正なカード".to_string());
        let error_str: &str = error.as_ref();
        assert_eq!(error_str, "カードエラー");
        
        let error = DomainError::InvalidGameOperation("操作エラー".to_string());
        let error_str: &str = error.as_ref();
        assert_eq!(error_str, "ゲーム操作エラー");
    }

    #[test]
    fn ドメインエラーから静的文字列への変換() {
        let error = DomainError::InvalidState("状態エラー".to_string());
        let error_str: &'static str = error.into();
        assert_eq!(error_str, "ドメインエラーが発生しました");
    }
} 