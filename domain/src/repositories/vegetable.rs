use async_trait::async_trait;

use crate::models::primitives::Price;
use crate::models::vegetable::{Vegetable, VegetableId};

/// 登録または更新する野菜
pub struct UpsertVegetable {
    /// 野菜名
    pub name: String,
    /// 単価
    pub unit_price: Price,
}

/// 部分更新する野菜
pub struct PartialVegetable {
    /// 野菜名
    pub name: Option<String>,
    /// 単価
    pub unit_price: Option<Price>,
}

/// 野菜リポジトリ
#[async_trait]
pub trait VegetableRepository: 'static {
    /// 野菜IDで指定した野菜を検索する。
    async fn find_by_id(&self, id: VegetableId) -> anyhow::Result<Option<Vegetable>>;

    /// すべての野菜を検索する。
    async fn find_all(&self) -> anyhow::Result<Vec<Vegetable>>;

    /// 野菜を登録する。
    async fn register(&self, vegetable: UpsertVegetable) -> anyhow::Result<Vegetable>;

    /// 野菜を更新する。
    async fn update(
        &self,
        id: VegetableId,
        vegetable: UpsertVegetable,
    ) -> anyhow::Result<Option<Vegetable>>;

    /// 野菜を部分更新する。
    async fn partial_update(
        &self,
        id: VegetableId,
        vegetable: PartialVegetable,
    ) -> anyhow::Result<Option<Vegetable>>;

    /// 野菜IDで指定した野菜を削除する。
    async fn delete(&self, id: VegetableId) -> anyhow::Result<u32>;
}
