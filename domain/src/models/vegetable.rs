use time::OffsetDateTime;
use uuid::Uuid;

use super::primitives::Price;
use macros::EntityId;

/// 野菜ID
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, EntityId)]
pub struct VegetableId {
    value: Uuid,
}

/// 野菜
pub struct Vegetable {
    /// 野菜ID
    id: VegetableId,
    /// 野菜名
    name: String,
    /// 単価
    unit_price: Price,
    /// 作成日時
    created_at: OffsetDateTime,
    /// 更新日時
    updated_at: OffsetDateTime,
}

impl Vegetable {
    /// 野菜を構築する。
    ///
    /// # 引数
    ///
    /// * `id` - 野菜ID
    /// * `name` - 野菜名
    /// * `unit_price` - 単価
    /// * `created_at` - 作成日時
    /// * `updated_at` - 更新日時
    ///
    /// # 戻り値
    ///
    /// 野菜
    pub fn new(
        id: VegetableId,
        name: &str,
        unit_price: Price,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            unit_price,
            created_at,
            updated_at,
        }
    }

    /// 野菜IDを返す。
    ///
    /// # 戻り値
    ///
    /// 野菜ID
    pub fn id(&self) -> VegetableId {
        self.id
    }

    /// 野菜名を返す。
    ///
    /// # 戻り値
    ///
    /// 野菜名
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 単価を返す。
    ///
    /// # 戻り値
    ///
    /// 単価
    pub fn unit_price(&self) -> Price {
        self.unit_price
    }

    /// 作成日時を返す。
    ///
    /// # 戻り値
    ///
    /// 作成日時
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    /// 更新日時を返す。
    ///
    /// # 戻り値
    ///
    /// 更新日時
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }
}
