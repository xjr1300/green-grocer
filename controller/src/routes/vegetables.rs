use actix_web::{web, HttpResponse, Scope};
use uuid::Uuid;

use super::{e500, HandlerReturnType};
use domain::repositories::vegetable::{UpsertVegetable, VegetableRepository};
use domain::repositories::RepositoryContainer;
use infrastructure::postgres::repositories::vegetable::PlainVegetable;

pub fn vegetable_router<VR>() -> Scope
where
    VR: VegetableRepository,
{
    web::scope("/api/vegetables")
        .route("", web::get().to(find_all::<VR>))
        .route("", web::post().to(register::<VR>))
}

/// 野菜をすべて検索するハンドラ関数
///
/// # 引数
///
/// * `repo_container` - リポジトリコンテナ
///
/// # 戻り値
///
/// レスポンス
async fn find_all<VR>(repo_container: web::Data<RepositoryContainer<VR>>) -> HandlerReturnType
where
    VR: VegetableRepository,
{
    let vegetables: Vec<PlainVegetable> = repo_container
        .vegetable
        .find_all()
        .await
        .map_err(e500)?
        .into_iter()
        .map(|v| v.into())
        .collect();

    Ok(HttpResponse::Ok().json(vegetables))
}

/// プレインな野菜登録または更新データ
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlainUpsertVegetable {
    /// 野菜ID
    pub id: Uuid,
    /// 野菜名
    pub name: String,
    /// 単価
    pub unit_price: u32,
}

impl From<PlainUpsertVegetable> for UpsertVegetable {
    fn from(value: PlainUpsertVegetable) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            unit_price: value.unit_price.into(),
        }
    }
}

/// 野菜を登録するハンドラ関数
///
/// # 引数
///
/// * `repo_container` - リポジトリコンテナ
/// * `vegetable` - 野菜
///
/// # 戻り値
///
/// レスポンス
async fn register<VR>(
    repo_container: web::Data<RepositoryContainer<VR>>,
    vegetable: web::Json<PlainUpsertVegetable>,
) -> HandlerReturnType
where
    VR: VegetableRepository,
{
    let vegetable: UpsertVegetable = vegetable.into_inner().into();
    let vegetable = repo_container
        .vegetable
        .register(vegetable)
        .await
        .map_err(e500)?;
    let vegetable: PlainVegetable = vegetable.into();

    Ok(HttpResponse::Ok().json(vegetable))
}
