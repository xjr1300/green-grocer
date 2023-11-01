use actix_web::{web, HttpResponse, Scope};

use super::{e400, e404, e500, HandlerReturnType};
use domain::models::primitives::Price;
use domain::models::vegetable::VegetableId;
use domain::repositories::vegetable::{PartialVegetable, UpsertVegetable, VegetableRepository};
use domain::repositories::RepositoryContainer;
use infrastructure::postgres::repositories::vegetable::PlainVegetable;

pub fn vegetable_router<VR>() -> Scope
where
    VR: VegetableRepository,
{
    web::scope("/api/vegetables")
        .route("", web::get().to(find_all::<VR>))
        .route("", web::post().to(register::<VR>))
        .route("/{id}", web::get().to(find_by_id::<VR>))
        .route("/{id}", web::put().to(update::<VR>))
        .route("/{id}", web::patch().to(partial_update::<VR>))
        .route("/{id}", web::delete().to(delete::<VR>))
}

/// 野菜をすべて検索するハンドラ関数
///
/// [GET] http://localhost:8001/api/vegetables
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

/// 野菜をIDで検索するハンドラ関数
///
/// [GET] http://localhost:8001/api/vegetables/{id}
///
/// # 引数
///
/// * `repo_container` - リポジトリコンテナ
///
/// # 戻り値
///
/// レスポンス
async fn find_by_id<VR>(
    repo_container: web::Data<RepositoryContainer<VR>>,
    path: web::Path<(String,)>,
) -> HandlerReturnType
where
    VR: VegetableRepository,
{
    let id: VegetableId = VegetableId::try_from(path.into_inner().0.as_str()).map_err(e400)?;
    let vegetable = repo_container
        .vegetable
        .find_by_id(id)
        .await
        .map_err(e500)?;
    if vegetable.is_none() {
        return Err(e404());
    }
    let vegetable: PlainVegetable = vegetable.unwrap().into();

    Ok(HttpResponse::Ok().json(vegetable))
}

/// プレインな野菜登録または更新データ
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlainUpsertVegetable {
    /// 野菜名
    pub name: String,
    /// 単価
    pub unit_price: u32,
}

impl From<PlainUpsertVegetable> for UpsertVegetable {
    fn from(value: PlainUpsertVegetable) -> Self {
        Self {
            name: value.name,
            unit_price: value.unit_price.into(),
        }
    }
}

/// 野菜を登録するハンドラ関数
///
/// [POST] http://localhost:8001/api/vegetables
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

/// 野菜を更新するハンドラ関数
///
/// [PUT] http://localhost:8001/api/vegetables/{id}
///
///
/// * `repo_container` - リポジトリコンテナ
/// * `vegetable` - 野菜
///
/// # 戻り値
///
/// レスポンス
async fn update<VR>(
    repo_container: web::Data<RepositoryContainer<VR>>,
    path: web::Path<(String,)>,
    vegetable: web::Json<PlainUpsertVegetable>,
) -> HandlerReturnType
where
    VR: VegetableRepository,
{
    let id: VegetableId = VegetableId::try_from(path.into_inner().0.as_str()).map_err(e400)?;
    let vegetable: UpsertVegetable = vegetable.into_inner().into();
    let vegetable = repo_container
        .vegetable
        .update(id, vegetable)
        .await
        .map_err(e500)?;
    if vegetable.is_none() {
        return Err(e404());
    }
    let vegetable: PlainVegetable = vegetable.unwrap().into();

    Ok(HttpResponse::Ok().json(vegetable))
}

/// プレインな野菜部分更新データ
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlainPartialVegetable {
    /// 野菜名
    pub name: Option<String>,
    /// 単価
    pub unit_price: Option<u32>,
}

impl From<PlainPartialVegetable> for PartialVegetable {
    fn from(value: PlainPartialVegetable) -> Self {
        Self {
            name: value.name,
            unit_price: value.unit_price.map(Price::from),
        }
    }
}

/// 野菜を部分更新するハンドラ関数
///
/// [PATCH] http://localhost:8001/api/vegetables/{id}
///
///
/// * `repo_container` - リポジトリコンテナ
/// * `vegetable` - 野菜
///
/// # 戻り値
///
/// レスポンス
async fn partial_update<VR>(
    repo_container: web::Data<RepositoryContainer<VR>>,
    path: web::Path<(String,)>,
    vegetable: web::Json<PlainPartialVegetable>,
) -> HandlerReturnType
where
    VR: VegetableRepository,
{
    let id: VegetableId = VegetableId::try_from(path.into_inner().0.as_str()).map_err(e400)?;
    let vegetable: PartialVegetable = vegetable.into_inner().into();
    let vegetable = repo_container
        .vegetable
        .partial_update(id, vegetable)
        .await
        .map_err(e500)?;
    if vegetable.is_none() {
        return Err(e404());
    }
    let vegetable: PlainVegetable = vegetable.unwrap().into();

    Ok(HttpResponse::Ok().json(vegetable))
}

/// 野菜をIDを指定して削除する関数
///
/// [DELETE] http://localhost:8001/api/vegetables/{id}
///
///
/// * `repo_container` - リポジトリコンテナ
/// * `id` - 野菜ID
///
/// # 戻り値
///
/// レスポンス
async fn delete<VR>(
    repo_container: web::Data<RepositoryContainer<VR>>,
    path: web::Path<(String,)>,
) -> HandlerReturnType
where
    VR: VegetableRepository,
{
    let id: VegetableId = VegetableId::try_from(path.into_inner().0.as_str()).map_err(e400)?;
    match repo_container.vegetable.delete(id).await.map_err(e500)? {
        0 => Err(e404()),
        _ => Ok(HttpResponse::Ok().finish()),
    }
}
