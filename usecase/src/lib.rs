pub mod interactors;

use std::borrow::Cow;

use domain::DomainError;

#[derive(thiserror::Error, Debug)]
pub enum UsecaseError {
    /// バリデーションエラー
    #[error("バリデーションエラー: {0}")]
    Validation(Cow<'static, str>),

    /// ドメインルールエラー
    #[error("ドメインルールエラー: {0}")]
    DomainRule(Cow<'static, str>),

    /// 予期しないエラー
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl From<DomainError> for UsecaseError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::Validation(message) => Self::Validation(message),
            DomainError::Unexpected(error) => Self::Unexpected(error),
        }
    }
}

/// ユースケース結果
pub type UsecaseResult<T> = Result<T, UsecaseError>;
