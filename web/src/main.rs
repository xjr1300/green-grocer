use actix_web::{App, HttpServer};

use controller::health_check::health_check;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8001))?
        .run()
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
