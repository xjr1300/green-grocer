use std::fmt;
use std::hash::{Hash, Hasher};

use time::OffsetDateTime;

use super::primitives::EntityId;

/// 販売ID
pub type SaleId = EntityId<Sale>;

impl Clone for SaleId {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for SaleId {}

impl PartialEq for SaleId {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl Eq for SaleId {}

impl Hash for SaleId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().hash(state);
    }
}

impl fmt::Debug for SaleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SaleId({})", self.value())
    }
}

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
    pub fn id(&self) -> SaleId {
        self.id
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
