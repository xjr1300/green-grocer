use actix_web::{web, App, HttpServer};

use controller::health_check::health_check;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    // データベース接続プールを作成
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    // Webアプリケーションサーバを起動
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
