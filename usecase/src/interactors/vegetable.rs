use async_trait::async_trait;

use crate::UsecaseResult;
use domain::models::vegetable::Vegetable;
use domain::repositories::vegetable::{PartialVegetable, UpsertVegetable};

/// 登録または更新する野菜
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertVegetableInput {
    /// 野菜の名前
    pub name: String,
    /// 野菜の単価
    pub unit_price: u32,
}

impl From<UpsertVegetableInput> for UpsertVegetable {
    fn from(value: UpsertVegetableInput) -> Self {
        Self {
            name: value.name,
            unit_price: value.unit_price.into(),
        }
    }
}

/// 部分更新する野菜
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialVegetableInput {
    /// 野菜名
    pub name: Option<String>,
    /// 単価
    pub unit_price: Option<u32>,
}

impl From<PartialVegetableInput> for PartialVegetable {
    fn from(value: PartialVegetableInput) -> Self {
        Self {
            name: value.name,
            unit_price: value.unit_price.map(|value| value.into()),
        }
    }
}

/// 野菜ユースケースインタラクター
#[async_trait]
pub trait VegetableInteractor: Clone {
    /// 野菜IDで指定された野菜を検索する。
    async fn find_by_id(&self, id: &str) -> UsecaseResult<Option<Vegetable>>;

    /// すべての野菜を検索する。
    async fn find_all(&self) -> UsecaseResult<Vec<Vegetable>>;

    /// 野菜を登録する。
    async fn register(&self, input: UpsertVegetableInput) -> UsecaseResult<Vegetable>;

    /// 野菜を更新する。
    async fn update(
        &self,
        id: &str,
        vegetable: UpsertVegetableInput,
    ) -> UsecaseResult<Option<Vegetable>>;

    /// 野菜を部分更新する。
    async fn partial_update(
        &self,
        id: &str,
        vegetable: PartialVegetableInput,
    ) -> UsecaseResult<Option<Vegetable>>;

    /// 野菜IDで指定した野菜を削除する。
    async fn delete(&self, id: &str) -> UsecaseResult<u32>;
}
