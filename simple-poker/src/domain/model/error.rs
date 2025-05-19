use thiserror::Error;
use crate::domain::model::game::GamePhase;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("無効なカード: {0}")]
    InvalidCard(String),
    
    #[error("無効なゲーム操作: {0}")]
    InvalidGameOperation(String),
    
    #[error("無効なプレイヤー操作: {0}")]
    InvalidPlayerOperation(String),
    
    #[error("無効なベット: {0}")]
    InvalidBet(String),
    
    #[error("リソースが見つかりません: {0}")]
    ResourceNotFound(String),
    
    #[error("不正な状態: {0}")]
    InvalidState(String),

    #[error("フェーズエラー: 現在の{actual:?}フェーズでは{expected:?}操作は許可されていません")]
    InvalidPhase {
        expected: GamePhase,
        actual: GamePhase,
    },
    
    #[error("ベット制約エラー: {message}")]
    BettingConstraint { message: String },
    
    #[error("プレイヤーエラー: {0}")]
    PlayerError(#[from] PlayerError),
    
    #[error("デッキエラー: {0}")]
    DeckError(#[from] DeckError),
}

/// プレイヤー関連の特化したエラー
#[derive(Debug, Error)]
pub enum PlayerError {
    #[error("資金不足: 必要額 {required}、所持チップ {available}")]
    InsufficientFunds { required: u32, available: u32 },
    
    #[error("プレイヤーは既にフォールドしています")]
    AlreadyFolded,
    
    #[error("プレイヤーは既にオールインしています")]
    AlreadyAllIn,
    
    #[error("無効なプレイヤー操作: {0}")]
    InvalidOperation(String),
}

/// デッキ関連の特化したエラー
#[derive(Debug, Error)]
pub enum DeckError {
    #[error("デッキが空です")]
    EmptyDeck,
    
    #[error("無効なカードインデックス: {0}")]
    InvalidCardIndex(usize),
    
    #[error("無効なカード操作: {0}")]
    InvalidOperation(String),
}

// 文字列からDomainErrorへの変換を便利にするためのFrom実装
impl From<&'static str> for DomainError {
    fn from(s: &'static str) -> Self {
        DomainError::InvalidState(s.to_string())
    }
}

// Stringからの変換
impl From<String> for DomainError {
    fn from(s: String) -> Self {
        DomainError::InvalidState(s)
    }
}

// DomainErrorからStringへの変換
impl From<DomainError> for String {
    fn from(error: DomainError) -> Self {
        error.to_string()
    }
}

// &DomainErrorからStringへの変換
impl From<&DomainError> for String {
    fn from(error: &DomainError) -> Self {
        error.to_string()
    }
}

// DomainErrorをAsRef<str>として扱えるようにする
impl AsRef<str> for DomainError {
    fn as_ref(&self) -> &str {
        match self {
            Self::InvalidCard(s) => s.as_ref(),
            Self::InvalidGameOperation(s) => s.as_ref(),
            Self::InvalidPlayerOperation(s) => s.as_ref(),
            Self::InvalidBet(s) => s.as_ref(),
            Self::ResourceNotFound(s) => s.as_ref(),
            Self::InvalidState(s) => s.as_ref(),
            _ => "エラーが発生しました",
        }
    }
}

// &'static strからDomainErrorへの変換（&str -> DomainError）
impl<'a> From<DomainError> for &'a str {
    fn from(_error: DomainError) -> &'a str {
        "エラーが発生しました"
    }
} 