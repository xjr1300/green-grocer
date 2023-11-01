use crate::DomainError;

/// 価格
///
/// 0以上の整数を持つ価格を表現する。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price(u32);

impl Price {
    /// 価格を返す。
    ///
    /// # 戻り値
    ///
    /// 価格
    pub fn value(&self) -> u32 {
        self.0
    }
}

macro_rules! price_from_signed_integers {
    ($target:ty) => {
        impl TryFrom<$target> for Price {
            type Error = DomainError;

            /// 整数から価格を構築する。
            ///
            /// # 引数
            ///
            /// * `value` - 価格
            ///
            /// # 戻り値
            ///
            /// 価格
            ///
            /// # エラー
            ///
            /// `DomainError::Rule`
            fn try_from(value: $target) -> Result<Self, Self::Error> {
                if value < 0 as $target {
                    return Err(DomainError::Rule("数量は0以上です。".into()));
                }

                Ok(Self { 0: value as u32 })
            }
        }
    };
}

macro_rules! price_from_unsigned_integers {
    ($target:ty) => {
        impl From<$target> for Price {
            /// 整数から価格を構築する。
            ///
            /// # 引数
            ///
            /// * `value` - 価格
            ///
            /// # 戻り値
            ///
            /// 価格
            ///
            /// # エラー
            ///
            /// `DomainError::Rule`
            fn from(value: $target) -> Self {
                Self { 0: value as u32 }
            }
        }
    };
}

price_from_signed_integers!(i8);
price_from_signed_integers!(i16);
price_from_signed_integers!(i32);
price_from_unsigned_integers!(u8);
price_from_unsigned_integers!(u16);
price_from_unsigned_integers!(u32);

/// 数量
///
/// 1以上の整数を持つ数量を表現する。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quantity(u32);

impl Quantity {
    /// 数量を返す。
    ///
    /// # 戻り値
    ///
    /// 数量
    pub fn value(&self) -> u32 {
        self.0
    }
}

macro_rules! quantity_from_integers {
    ($target:ty) => {
        impl TryFrom<$target> for Quantity {
            type Error = DomainError;

            /// 整数から数量を構築する。
            ///
            /// # 引数
            ///
            /// * `value` - 数量を構築する整数
            ///
            /// # 戻り値
            ///
            /// 数量
            ///
            /// # エラー
            ///
            /// `DomainError::Rule`
            fn try_from(value: $target) -> Result<Self, Self::Error> {
                if value < 1 as $target {
                    return Err(DomainError::Rule("数量は1以上です。".into()));
                }

                Ok(Self { 0: value as u32 })
            }
        }
    };
}

quantity_from_integers!(i8);
quantity_from_integers!(i16);
quantity_from_integers!(i32);
quantity_from_integers!(u8);
quantity_from_integers!(u16);
quantity_from_integers!(u32);
