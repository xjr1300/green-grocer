use std::hash::Hash;

use time::OffsetDateTime;
use uuid::Uuid;

use super::primitives::{Price, Quantity};
use super::vegetable::Vegetable;
use macros::EntityId;

/// 販売ID
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, EntityId)]
pub struct SaleId {
    value: Uuid,
}

/// 販売
pub struct Sale {
    /// 販売ID
    id: SaleId,
    /// 販売日時
    sold_at: OffsetDateTime,
    /// 販売明細
    sale_details: Vec<SaleDetail>,
    /// 合計販売金額
    total_price: u32,
    /// 作成日時
    created_at: OffsetDateTime,
    /// 更新日時
    updated_at: Option<OffsetDateTime>,
}

impl Sale {
    /// 販売を構築する。
    ///
    /// # 引数
    ///
    /// * `sold_at` - 販売日時
    /// * `sale_details` - 販売明細
    ///
    /// # 戻り値
    ///
    /// 販売
    pub fn new(sold_at: OffsetDateTime, sale_details: Vec<SaleDetail>) -> Self {
        let total_price = sale_details
            .iter()
            .map(|sd| sd.sold_unit_price().value() * sd.sold_quantity().value())
            .sum();

        Self {
            id: SaleId::default(),
            sold_at,
            sale_details,
            total_price,
            created_at: OffsetDateTime::now_utc(),
            updated_at: None,
        }
    }

    /// 販売IDを返す。
    ///
    /// # 戻り値
    ///
    /// 販売ID。
    pub fn id(&self) -> SaleId {
        self.id
    }

    /// 販売日時を返す。
    ///
    /// # 戻り値
    ///
    /// 販売日時。
    pub fn sold_at(&self) -> OffsetDateTime {
        self.sold_at
    }

    /// 販売明細を返す。
    ///
    /// # 戻り値
    ///
    /// 販売明細を格納したベクタ
    pub fn sale_details(&self) -> &[SaleDetail] {
        &self.sale_details
    }

    /// 合計販売金額を返す。
    ///
    /// # 戻り値
    ///
    /// 合計販売金額。
    pub fn total_price(&self) -> u32 {
        self.total_price
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
    pub fn updated_at(&self) -> Option<OffsetDateTime> {
        self.updated_at
    }
}

/// 販売明細ID
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, EntityId)]
pub struct SaleDetailId {
    value: Uuid,
}

/// 販売明細
pub struct SaleDetail {
    /// 販売明細ID
    id: SaleDetailId,
    /// 販売した野菜
    vegetable: Vegetable,
    /// 野菜を販売した単価
    sold_unit_price: Price,
    /// 野菜を販売した数量
    sold_quantity: Quantity,
}

impl SaleDetail {
    /// 販売明細を構築する。
    ///
    /// # 引数
    ///
    /// * `vegetable` - 販売した野菜
    /// * `sold_unit_price` - 野菜を販売した単価
    /// * `sold_quantity` - 野菜を販売した数量
    ///
    /// # 戻り値
    pub fn new(vegetable: Vegetable, sold_unit_price: Price, sold_quantity: Quantity) -> Self {
        Self {
            id: SaleDetailId::default(),
            vegetable,
            sold_unit_price,
            sold_quantity,
        }
    }

    /// 販売明細IDを返す。
    ///
    /// # 戻り値
    ///
    /// 販売明細ID
    pub fn id(&self) -> SaleDetailId {
        self.id
    }

    /// 販売した野菜を返す。
    ///
    /// # 戻り値
    ///
    /// 販売した野菜
    pub fn vegetable(&self) -> &Vegetable {
        &self.vegetable
    }

    /// 野菜を販売した単価を返す。
    ///
    /// # 戻り値
    ///
    /// 野菜を販売した時の単価
    pub fn sold_unit_price(&self) -> Price {
        self.sold_unit_price
    }

    /// 野菜を販売した数量を返す。
    ///
    /// # 戻り値
    ///
    /// 野菜を販売した数量
    pub fn sold_quantity(&self) -> Quantity {
        self.sold_quantity
    }
}
