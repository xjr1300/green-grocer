use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

use controller::health_check::health_check;
use domain::repositories::RepositoryContainer;
use infrastructure::postgres::repositories::vegetable::PgVegetableRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    // データベース接続プールを作成
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    let repo_container = RepositoryContainer {
        vegetable: PgVegetableRepository::new(pool.clone()),
    };
    let repo_container = web::Data::new(repo_container.clone());

    // Webアプリケーションサーバを起動
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(repo_container.clone()))
            .service(health_check)
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
