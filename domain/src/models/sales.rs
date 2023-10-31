use time::OffsetDateTime;

use super::primitives::EntityId;

/// 販売ID
pub type SaleId = EntityId<Sale>;

/// 販売
pub struct Sale {
    /// 販売ID
    id: SaleId,
    /// 販売日時
    sold_at: OffsetDateTime,
    /// 合計販売金額
    total_amount: u32,
}

impl Sale {
    /// 販売IDを返却する。
    ///
    /// # 戻り値
    ///
    /// 販売ID。
    pub fn id(&self) -> &SaleId {
        &self.id
    }

    /// 販売日時を返却する。
    ///
    /// # 戻り値
    ///
    /// 販売日時。
    pub fn sold_at(&self) -> OffsetDateTime {
        self.sold_at
    }

    /// 合計販売金額を返却する。
    ///
    /// # 戻り値
    ///
    /// 合計販売金額。
    pub fn total_amount(&self) -> u32 {
        self.total_amount
    }
}
