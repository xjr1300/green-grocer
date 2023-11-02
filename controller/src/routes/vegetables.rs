use actix_web::{web, HttpResponse, Scope};
use usecase::interactors::UsecaseInteractorContainer;

use super::{e404, e500, HandlerReturnType};
use infrastructure::postgres::PlainVegetable;
use usecase::interactors::vegetable::{
    PartialVegetableInput, UpsertVegetableInput, VegetableInteractor,
};

pub fn vegetable_router<VI>() -> Scope
where
    VI: VegetableInteractor + 'static,
{
    web::scope("/api/vegetables")
        .route("", web::get().to(find_all::<VI>))
        .route("", web::post().to(register::<VI>))
        .route("/{id}", web::get().to(find_by_id::<VI>))
        .route("/{id}", web::put().to(update::<VI>))
        .route("/{id}", web::patch().to(partial_update::<VI>))
        .route("/{id}", web::delete().to(delete::<VI>))
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
async fn find_all<VI>(
    repo_container: web::Data<UsecaseInteractorContainer<VI>>,
) -> HandlerReturnType
where
    VI: VegetableInteractor,
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
async fn find_by_id<VI>(
    repo_container: web::Data<UsecaseInteractorContainer<VI>>,
    path: web::Path<(String,)>,
) -> HandlerReturnType
where
    VI: VegetableInteractor,
{
    let vegetable = repo_container
        .vegetable
        .find_by_id(&path.into_inner().0)
        .await
        .map_err(e500)?;
    if vegetable.is_none() {
        return Err(e404());
    }
    let vegetable: PlainVegetable = vegetable.unwrap().into();

    Ok(HttpResponse::Ok().json(vegetable))
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
async fn register<VI>(
    repo_container: web::Data<UsecaseInteractorContainer<VI>>,
    vegetable: web::Json<UpsertVegetableInput>,
) -> HandlerReturnType
where
    VI: VegetableInteractor,
{
    let vegetable = repo_container
        .vegetable
        .register(vegetable.into_inner())
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
async fn update<VI>(
    repo_container: web::Data<UsecaseInteractorContainer<VI>>,
    path: web::Path<(String,)>,
    vegetable: web::Json<UpsertVegetableInput>,
) -> HandlerReturnType
where
    VI: VegetableInteractor,
{
    let vegetable = repo_container
        .vegetable
        .update(&path.into_inner().0, vegetable.into_inner())
        .await
        .map_err(e500)?;
    if vegetable.is_none() {
        return Err(e404());
    }
    let vegetable: PlainVegetable = vegetable.unwrap().into();

    Ok(HttpResponse::Ok().json(vegetable))
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
async fn partial_update<VI>(
    repo_container: web::Data<UsecaseInteractorContainer<VI>>,
    path: web::Path<(String,)>,
    vegetable: web::Json<PartialVegetableInput>,
) -> HandlerReturnType
where
    VI: VegetableInteractor,
{
    let vegetable = repo_container
        .vegetable
        .partial_update(&path.into_inner().0, vegetable.into_inner())
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
async fn delete<VI>(
    repo_container: web::Data<UsecaseInteractorContainer<VI>>,
    path: web::Path<(String,)>,
) -> HandlerReturnType
where
    VI: VegetableInteractor,
{
    match repo_container
        .vegetable
        .delete(path.into_inner().0.as_str())
        .await
        .map_err(e500)?
    {
        0 => Err(e404()),
        _ => Ok(HttpResponse::Ok().finish()),
    }
}
