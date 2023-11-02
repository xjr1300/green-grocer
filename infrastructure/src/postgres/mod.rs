pub mod interactors;
pub mod repositories;

use domain::models::vegetable::Vegetable;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlainVegetable {
    id: Uuid,
    name: String,
    unit_price: i32,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    updated_at: OffsetDateTime,
}

impl From<PlainVegetable> for Vegetable {
    fn from(value: PlainVegetable) -> Self {
        // 永続化層からのデータはドメインルールを満たしていることを前提とするため、
        // エラー処理を省略
        Self::new(
            value.id.into(),
            &value.name,
            value.unit_price.try_into().unwrap(),
            value.created_at,
            value.updated_at,
        )
    }
}

impl From<Vegetable> for PlainVegetable {
    fn from(value: Vegetable) -> Self {
        Self {
            id: value.id().value(),
            name: value.name().to_string(),
            unit_price: value.unit_price().value() as i32,
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}
