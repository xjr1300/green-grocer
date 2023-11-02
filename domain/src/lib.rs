pub mod models;
pub mod repositories;

use std::borrow::Cow;

/// ドメインエラー
#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    /// バリデーションエラー
    #[error("バリデーションエラー: {0}")]
    Validation(Cow<'static, str>),

    /// 予期しないエラー
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

/// ドメイン結果
pub type DomainResult<T> = Result<T, DomainError>;
