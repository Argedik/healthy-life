use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
  let pool = SqlitePoolOptions::new()
    .connect("sqlite://../database/sqlite.db")
    .await
    .expect("Veritabanına bağlanılamadı");

  // // Eğer migrations klasörü varsa, çalıştırın. Yoksa bu kodu yorum satırına alın.
  // if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
  //     eprintln!("Migration hatası: {:?}", e);
  // }
  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .unwrap();

  pool
}