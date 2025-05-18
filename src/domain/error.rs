use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum GameError {
    #[error("ゲームが見つかりません: {0}")]
    GameNotFound(String),
    
    #[error("ゲームが既に開始されています")]
    GameAlreadyStarted,
    
    #[error("プレイヤーが見つかりません: {0}")]
    PlayerNotFound(String),
    
    #[error("無効なゲームフェーズです: {expected}が必要ですが、{actual}です")]
    InvalidGamePhase { expected: String, actual: String },
    
    #[error("無効なプレイヤーインデックスです: {0}")]
    InvalidPlayerIndex(usize),
    
    #[error("無効なプレイヤー数です: {0}")]
    InvalidPlayerCount(usize),
    
    #[error("無効なベット額です: {0}")]
    InvalidBetAmount(u32),
    
    #[error("無効なカードインデックスです: {0}")]
    InvalidCardIndex(usize),
    
    #[error("無効なアクションです: {0}")]
    InvalidAction(String),
    
    #[error("チップが足りません: 必要={required}, 所持={available}")]
    InsufficientChips { required: u32, available: u32 },
    
    #[error("デッキが空です")]
    DeckEmpty,
    
    #[error("ファイル操作エラー: {0}")]
    FileOperationError(String),
    
    #[error("シリアライズエラー: {0}")]
    SerializationError(String),
    
    #[error("デシリアライズエラー: {0}")]
    DeserializationError(String),
    
    #[error("その他のエラー: {0}")]
    Other(String),
}

#[derive(Error, Debug, Clone)]
pub enum RepositoryError {
    #[error("保存エラー: {0}")]
    SaveError(String),
    
    #[error("読み込みエラー: {0}")]
    ReadError(String),
    
    #[error("削除エラー: {0}")]
    DeleteError(String),
    
    #[error("ロックエラー: {0}")]
    LockError(String),
    
    #[error("その他のエラー: {0}")]
    Other(String),
}

impl From<std::io::Error> for RepositoryError {
    fn from(err: std::io::Error) -> Self {
        RepositoryError::Other(err.to_string())
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(err: serde_json::Error) -> Self {
        RepositoryError::Other(err.to_string())
    }
}

pub type GameResult<T> = Result<T, GameError>;
pub type RepositoryResult<T> = Result<T, RepositoryError>; 