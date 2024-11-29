use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
  let pool = SqlitePoolOptions::new()
    .connect("sqlite://../database/sqlite.db")
    .await
    .unwrap();

  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .unwrap()

  pool
}