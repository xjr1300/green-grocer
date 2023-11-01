use std::fmt::{Debug, Display};

use actix_web::HttpResponse;

pub mod vegetables;

pub type HandlerReturnType = Result<HttpResponse, actix_web::error::Error>;

/// 500 Internal Server Errorを作成する。
///
/// # Arguments
///
/// * `err` - エラー
///
/// # Returns
///
/// `actix_web::error::ErrorBadRequest`
pub fn e500<E>(err: E) -> actix_web::Error
where
    E: Debug + Display + 'static,
{
    actix_web::error::ErrorInternalServerError(err)
}
