use thiserror::Error;

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
}

impl From<&'static str> for DomainError {
    fn from(s: &'static str) -> Self {
        DomainError::InvalidState(s.to_string())
    }
}

impl From<String> for DomainError {
    fn from(s: String) -> Self {
        DomainError::InvalidState(s)
    }
}

impl From<DomainError> for String {
    fn from(error: DomainError) -> Self {
        error.to_string()
    }
}

impl From<&DomainError> for String {
    fn from(error: &DomainError) -> Self {
        error.to_string()
    }
}

impl AsRef<str> for DomainError {
    fn as_ref(&self) -> &str {
        match self {
            DomainError::InvalidCard(s) => s.as_ref(),
            DomainError::InvalidGameOperation(s) => s.as_ref(),
            DomainError::InvalidPlayerOperation(s) => s.as_ref(),
            DomainError::InvalidBet(s) => s.as_ref(),
            DomainError::ResourceNotFound(s) => s.as_ref(),
            DomainError::InvalidState(s) => s.as_ref(),
        }
    }
}

impl<'a> From<DomainError> for &'a str {
    fn from(_error: DomainError) -> &'a str {
        "ドメインエラーが発生しました"
    }
}