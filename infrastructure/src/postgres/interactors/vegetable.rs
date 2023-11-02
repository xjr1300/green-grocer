use async_trait::async_trait;
use sqlx::PgPool;

use crate::postgres::repositories::vegetable::PgVegetableRepository;
use domain::models::vegetable::{Vegetable, VegetableId};
use domain::repositories::vegetable::{PartialVegetable, UpsertVegetable, VegetableRepository};
use usecase::interactors::vegetable::{
    PartialVegetableInput, UpsertVegetableInput, VegetableInteractor,
};
use usecase::{UsecaseError, UsecaseResult};

/// PostgreSQL用の野菜インタラクター
#[derive(Clone)]
pub struct PgVegetableInteractor {
    pool: PgPool,
}

impl PgVegetableInteractor {
    /// コンストラクタ
    ///
    /// # 引数
    ///
    /// * `pool` - データベース接続プール
    ///
    /// # 戻り値
    ///
    /// 野菜インタラクター
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VegetableInteractor for PgVegetableInteractor {
    /// 野菜IDで指定した野菜を検索する。
    ///
    /// # 引数
    ///
    /// * `id` - 野菜ID
    ///
    /// # 戻り値
    ///
    /// 野菜
    ///
    /// # エラー
    ///
    /// * `UsecaseError::Validation` - 引数の野菜IDがUUIDv4形式でない場合
    /// * `UsecaseError::Unexpected` - 予期しないエラーが発生した場合
    async fn find_by_id(&self, id: &str) -> UsecaseResult<Option<Vegetable>> {
        let id = convert_to_vegetable_id(id)?;
        let repo = PgVegetableRepository::new(self.pool.clone());

        repo.find_by_id(id).await.map_err(|e| e.into())
    }

    /// すべての野菜を検索する。
    ///
    ///
    /// # 戻り値
    ///
    /// 野菜を格納したベクタ
    ///
    /// # エラー
    ///
    /// * `UsecaseError::Unexpected` - 予期しないエラーが発生した場合
    async fn find_all(&self) -> UsecaseResult<Vec<Vegetable>> {
        PgVegetableRepository::new(self.pool.clone())
            .find_all()
            .await
            .map_err(|e| e.into())
    }

    /// 野菜を登録する。
    ///
    /// # 引数
    ///
    /// * `input` - 登録する野菜
    ///
    /// # 戻り値
    ///
    /// 登録した野菜
    ///
    /// # エラー
    ///
    /// * `UsecaseError::Unexpected` - 予期しないエラーが発生した場合
    async fn register(&self, input: UpsertVegetableInput) -> UsecaseResult<Vegetable> {
        let input: UpsertVegetable = input.into();

        PgVegetableRepository::new(self.pool.clone())
            .register(input)
            .await
            .map_err(|e| e.into())
    }

    /// 野菜を更新する。
    ///
    /// # 引数
    ///
    /// * `id` - 野菜ID
    /// * `input` - 更新する野菜
    ///
    /// # 戻り値
    ///
    /// 更新した野菜
    ///
    /// # エラー
    ///
    /// * `UsecaseError::Validation` - 引数の野菜IDがUUIDv4形式でない場合
    /// * `UsecaseError::Unexpected` - 予期しないエラーが発生した場合
    async fn update(
        &self,
        id: &str,
        input: UpsertVegetableInput,
    ) -> UsecaseResult<Option<Vegetable>> {
        let id = convert_to_vegetable_id(id)?;
        let input: UpsertVegetable = input.into();

        PgVegetableRepository::new(self.pool.clone())
            .update(id, input)
            .await
            .map_err(|e| e.into())
    }

    /// 野菜を部分更新する。
    ///
    /// # 引数
    ///
    /// * `id` - 野菜ID
    /// * `input` - 部分更新する野菜
    ///
    /// # 戻り値
    ///
    /// 部分更新した野菜
    ///
    /// # エラー
    ///
    /// * `UsecaseError::Validation` - 引数の野菜IDがUUIDv4形式でない場合
    /// * `UsecaseError::Unexpected` - 予期しないエラーが発生した場合
    async fn partial_update(
        &self,
        id: &str,
        input: PartialVegetableInput,
    ) -> UsecaseResult<Option<Vegetable>> {
        let id = convert_to_vegetable_id(id)?;
        let input: PartialVegetable = input.into();

        PgVegetableRepository::new(self.pool.clone())
            .partial_update(id, input)
            .await
            .map_err(|e| e.into())
    }

    /// 野菜IDで指定した野菜を削除する。
    async fn delete(&self, id: &str) -> UsecaseResult<u32> {
        let id = convert_to_vegetable_id(id)?;

        PgVegetableRepository::new(self.pool.clone())
            .delete(id)
            .await
            .map_err(|e| e.into())
    }
}

/// 文字列を野菜IDに変換する。
///
/// # 引数
///
/// * `id` - 野菜IDを表す文字列
///
/// # 戻り値
///
/// 野菜ID
///
/// # エラー
///
/// * `UsecaseError::Validation` - 引数の野菜IDがUUIDv4形式でない場合
fn convert_to_vegetable_id(id: &str) -> UsecaseResult<VegetableId> {
    id.try_into().map_err(|_| {
        UsecaseError::Validation("UUIDv4形式の文字列で野菜IDを指定してください。".into())
    })
}
