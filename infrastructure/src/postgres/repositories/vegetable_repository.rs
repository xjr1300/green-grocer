use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use time::OffsetDateTime;
use uuid::Uuid;

use domain::models::vegetable::{Vegetable, VegetableId};
use domain::repositories::vegetable_repository::{
    PartialVegetable, UpsertVegetable, VegetableRepository,
};

/// PostgreSQL用の野菜リポジトリ
#[derive(Clone, Debug)]
pub struct PgVegetableRepository {
    pool: PgPool,
}

#[derive(sqlx::FromRow)]
struct PlainVegetable {
    id: Uuid,
    name: String,
    unit_price: i32,
    created_at: OffsetDateTime,
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

impl PgVegetableRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VegetableRepository for PgVegetableRepository {
    /// 野菜IDで指定した野菜を検索する。
    ///
    /// # 引数
    ///
    /// * `id` - 野菜ID
    ///
    /// # 戻り値
    ///
    /// 野菜
    async fn find_by_id(&self, id: VegetableId) -> anyhow::Result<Option<Vegetable>> {
        let veg = sqlx::query_as!(
            PlainVegetable,
            r#"
            SELECT id, name, unit_price, created_at, updated_at
            FROM vegetables
            WHERE id = $1
            "#,
            id.value(),
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(veg.map(|v| v.into()))
    }

    /// すべての野菜を検索する。
    ///
    /// # 戻り値
    ///
    /// 野菜のベクタ
    async fn find_all(&self) -> anyhow::Result<Vec<Vegetable>> {
        let records = sqlx::query_as!(
            PlainVegetable,
            r#"
            SELECT id, name, unit_price, created_at, updated_at
            FROM vegetables
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records.into_iter().map(|v| v.into()).collect())
    }

    /// 野菜を登録する。
    ///
    /// # 引数
    ///
    /// * `vegetable` - 登録する野菜
    ///
    /// # 戻り値
    ///
    /// 登録した野菜
    async fn register(&self, vegetable: UpsertVegetable) -> anyhow::Result<Vegetable> {
        let mut tx = self.pool.begin().await?;
        let veg = {
            sqlx::query_as!(
                PlainVegetable,
                r#"
                INSERT INTO vegetables (id, name, unit_price, created_at, updated_at)
                VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                RETURNING id, name, unit_price, created_at, updated_at
                "#,
                vegetable.id.value(),
                &vegetable.name,
                vegetable.unit_price.value() as i32,
            )
            .fetch_one(&mut *tx)
            .await?
        };
        tx.commit().await?;

        Ok(veg.into())
    }

    /// 野菜を更新する。
    ///
    /// # 引数
    ///
    /// * `vegetable` - 更新する野菜
    ///
    /// # 戻り値
    ///
    /// 更新した野菜
    async fn update(&self, vegetable: UpsertVegetable) -> anyhow::Result<Option<Vegetable>> {
        let mut tx = self.pool.begin().await?;
        let veg = {
            sqlx::query_as!(
                PlainVegetable,
                r#"
                UPDATE vegetables
                SET name = $2, unit_price = $3, updated_at = CURRENT_TIMESTAMP
                WHERE id = $1
                RETURNING id, name, unit_price, created_at, updated_at
                "#,
                vegetable.id.value(),
                &vegetable.name,
                vegetable.unit_price.value() as i32,
            )
            .fetch_optional(&mut *tx)
            .await?
        };
        tx.commit().await?;

        match veg {
            Some(v) => Ok(Some(v.into())),
            None => Ok(None),
        }
    }

    /// 野菜を部分更新する。
    ///
    /// # 引数
    ///
    /// * `vegetable` - 部分更新する野菜
    ///
    /// # 戻り値
    ///
    /// 部分更新した野菜
    async fn partial_update(
        &self,
        vegetable: PartialVegetable,
    ) -> anyhow::Result<Option<Vegetable>> {
        if vegetable.name.is_none() && vegetable.unit_price.is_none() {
            return self.find_by_id(vegetable.id).await;
        }
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE vegetables SET");
        if let Some(name) = &vegetable.name {
            builder.push(" name = ");
            builder.push_bind(name);
        }
        if let Some(unit_price) = vegetable.unit_price {
            builder.push(" unit_price = ");
            builder.push_bind(unit_price.value() as i32);
        }
        builder.push(" updated_at = CURRENT_TIMESTAMP");
        builder.push(" WHERE id = ");
        builder.push_bind(vegetable.id.value());
        builder.push(" RETURNING id, name, unit_price, created_at, updated_at");

        let mut tx = self.pool.begin().await?;
        let veg = {
            builder
                .build_query_as::<PlainVegetable>()
                .fetch_optional(&mut *tx)
                .await?
        };
        tx.commit().await?;

        match veg {
            Some(v) => Ok(Some(v.into())),
            None => Ok(None),
        }
    }

    /// 野菜IDで指定した野菜を削除する。
    ///
    /// # 引数
    ///
    /// * `id` - 野菜ID
    ///
    /// # 戻り値
    ///
    /// 影響した行数。
    async fn delete(&self, id: VegetableId) -> anyhow::Result<u32> {
        let mut tx = self.pool.begin().await?;
        let result = {
            sqlx::query!(
                r#"
                DELETE FROM vegetables
                WHERE id = $1
                "#,
                id.value(),
            )
            .execute(&mut *tx)
            .await?
        };

        tx.commit().await?;

        Ok(result.rows_affected() as u32)
    }
}
