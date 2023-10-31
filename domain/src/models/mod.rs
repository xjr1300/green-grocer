use std::borrow::Cow;

pub mod primitives;

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("ドメインルール違反: {0}")]
    Rule(Cow<'static, str>),
}
