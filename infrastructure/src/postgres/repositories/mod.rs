use domain::{DomainError, DomainResult};
use sqlx::{Pool, Postgres, Transaction};

pub mod vegetable;

/// ドランザクションを開始する。
///
/// # 引数
///
/// * `pool` - データベース接続プール
///
/// # 戻り値
///
/// トランザクション
pub async fn begin_transaction(pool: &Pool<Postgres>) -> DomainResult<Transaction<'_, Postgres>> {
    pool.begin()
        .await
        .map_err(|e| DomainError::Unexpected(e.into()))
}

/// トランザクションをコミットする。
///
/// # 引数
///
/// * `tx` - トランザクション
///
/// # 戻り値
///
/// `()`
pub async fn commit_transaction(tx: Transaction<'_, Postgres>) -> DomainResult<()> {
    tx.commit()
        .await
        .map_err(|e| DomainError::Unexpected(e.into()))
}
