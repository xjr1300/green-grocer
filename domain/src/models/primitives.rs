use std::marker::PhantomData;

use uuid::Uuid;

use super::DomainError;

/// エンティティID構造体
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId<T> {
    value: Uuid,
    _marker: PhantomData<T>,
}

impl<T> EntityId<T> {
    /// エンティティIDインスタンスを構築する。
    ///
    /// # Returns
    ///
    /// エンティティIDインスタンス。
    fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
            _marker: PhantomData,
        }
    }

    /// エンティティIDインスタンスを構築する。
    ///
    /// # Arguments
    ///
    /// * value: IDに設定するUUID。
    ///
    /// # Returns
    ///
    /// エンティティIDインスタンス。
    fn from(value: Uuid) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// IDをUUIDで返却する。
    ///
    /// # Returns
    ///
    /// ID。
    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl<T> Default for EntityId<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Uuid> for EntityId<T> {
    fn from(value: Uuid) -> Self {
        Self::from(value)
    }
}

impl<T> TryFrom<&str> for EntityId<T> {
    type Error = DomainError;

    /// 文字列からエンティティIDを構築する。
    ///
    /// # Arguments:
    ///
    /// * `value` - エンティティIDを構築する文字列。
    ///
    /// # Returns
    ///
    /// エンティティIDインスタンス。
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = Uuid::try_parse(value).map_err(|err| {
            DomainError::Rule(format!("文字列をUUIDに変換できませんでした。{:?}", err).into())
        })?;

        Ok(Self::from(value))
    }
}
