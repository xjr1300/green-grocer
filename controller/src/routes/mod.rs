use std::fmt::{Debug, Display};

use actix_web::HttpResponse;

pub mod vegetables;

pub type HandlerReturnType = Result<HttpResponse, actix_web::error::Error>;

/// 400 Bad Request Errorを作成する。
///
/// # 引数
///
/// * `err` - エラー
///
/// # 戻り値
///
/// `actix_web::error::ErrorBadRequest`
pub fn e400<E>(err: E) -> actix_web::Error
where
    E: Debug + Display + 'static,
{
    actix_web::error::ErrorBadRequest(err)
}

pub fn e404() -> actix_web::Error {
    actix_web::error::ErrorNotFound("リソースが見つかりませんでした。")
}

/// 500 Internal Server Errorを作成する。
///
/// # 引数
///
/// * `err` - エラー
///
/// # 戻り値
///
/// `actix_web::error::ErrorBadRequest`
pub fn e500<E>(err: E) -> actix_web::Error
where
    E: Debug + Display + 'static,
{
    actix_web::error::ErrorInternalServerError(err)
}
