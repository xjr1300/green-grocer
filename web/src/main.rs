use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

use controller::health_check::health_check;
use controller::routes::vegetables::vegetable_router;
use infrastructure::postgres::interactors::vegetable::PgVegetableInteractor;
use usecase::interactors::UsecaseInteractorContainer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenvy::dotenv()?;

    // データベース接続プールを作成
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    // ユースケースインタラクターコンテナを構築
    let usecase_interactors = UsecaseInteractorContainer {
        vegetable: PgVegetableInteractor::new(pool.clone()),
    };

    // Webアプリケーションサーバを起動
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(usecase_interactors.clone()))
            .service(health_check)
            .service(vegetable_router::<PgVegetableInteractor>())
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
