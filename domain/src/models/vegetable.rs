use uuid::Uuid;

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
    unit_price: u32,
}

impl Vegetable {
    /// 野菜を構築する。
    ///
    /// # 引数
    ///
    /// * `name` - 野菜名
    /// * `unit_price` - 単価
    ///
    /// # 戻り値
    ///
    /// 野菜
    pub fn new(name: &str, unit_price: u32) -> Self {
        Self {
            id: VegetableId::default(),
            name: name.to_string(),
            unit_price,
        }
    }

    /// 野菜IDを返却する。
    ///
    /// # 戻り値
    ///
    /// 野菜ID。
    pub fn id(&self) -> VegetableId {
        self.id
    }

    /// 野菜名を返却する。
    ///
    /// # 戻り値
    ///
    /// 野菜名。
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 単価を返却する。
    ///
    /// # 戻り値
    ///
    /// 単価。
    pub fn unit_price(&self) -> u32 {
        self.unit_price
    }
}
