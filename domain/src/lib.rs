pub mod models;

use std::borrow::Cow;

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("ドメインルール違反: {0}")]
    Rule(Cow<'static, str>),
}
