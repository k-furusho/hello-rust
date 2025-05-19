use thiserror::Error;
use std::fmt;

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
        // 各エラー種別に応じた静的なメッセージを返す
        match self {
            DomainError::InvalidCard(_) => "カードエラー",
            DomainError::InvalidGameOperation(_) => "ゲーム操作エラー",
            DomainError::InvalidPlayerOperation(_) => "プレイヤー操作エラー",
            DomainError::InvalidBet(_) => "ベットエラー",
            DomainError::ResourceNotFound(_) => "リソース未発見エラー",
            DomainError::InvalidState(_) => "状態エラー",
        }
    }
}

// &'static strからDomainErrorへの変換（&str -> DomainError）
impl<'a> From<DomainError> for &'a str {
    fn from(_error: DomainError) -> &'a str {
        "ドメインエラーが発生しました"
    }
} 