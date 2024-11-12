use actix_web::{App, HttpServer};
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut cfg = Config::new();
    cfg.dbname = Some("beslenme_db".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let pool = cfg.create_pool(None, tokio_postgres::NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            // Rotalar burada eklenecek
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
