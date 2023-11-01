use crate::models::primitives::Price;
use crate::models::vegetable::{Vegetable, VegetableId};

/// 登録または更新する野菜
pub struct EditVegetable {
    /// 野菜ID
    id: VegetableId,
    /// 野菜名
    name: String,
    /// 単価
    unit_price: Price,
}

/// 野菜リポジトリ
pub trait VegetableRepository {
    /// 野菜IDで指定した野菜を検索する。
    fn find_by_id(&self, id: VegetableId) -> anyhow::Result<Vegetable>;
    /// すべての野菜を検索する。
    fn find_all(&self) -> anyhow::Result<Vec<Vegetable>>;
    /// 野菜を登録する。
    fn register(&self, vegetable: EditVegetable) -> anyhow::Result<Vegetable>;
    /// 野菜を更新する。
    fn update(&self, vegetable: EditVegetable) -> anyhow::Result<Vegetable>;
    /// 野菜IDで指定した野菜を削除する。
    fn delete(&self, id: VegetableId) -> anyhow::Result<()>;
}
