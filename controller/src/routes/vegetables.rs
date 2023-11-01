use actix_web::{web, HttpResponse, Scope};

use super::{e500, HandlerReturnType};
use domain::repositories::vegetable::VegetableRepository;
use domain::repositories::RepositoryContainer;
use infrastructure::postgres::repositories::vegetable::PlainVegetable;

pub fn vegetable_router<VR>() -> Scope
where
    VR: VegetableRepository + 'static,
{
    web::scope("/api/vegetables").route("", web::get().to(find_all::<VR>))
}

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
