// SqlitePool: SQLite veritabanı bağlantı havuzunu temsil eder.
// query ve query_as: SQL sorguları yazmak için kullanılan fonksiyonlar. query_as sorgu sonucunu direkt struct’a map eder.
use sqlx::{SqlitePool, query, query_as};

// Tek bir global değişkene (ör. static DB_POOL) uygulama boyunca erişebilmek için OnceCell kullanıyoruz. Bu sayede veritabanı bağlantı havuzunu bir kere oluşturup, her yerde kullanabiliyoruz.
use once_cell::sync::OnceCell; // Global pool tutmak için

// FridgeItem struct’ını kullanacağız (CRUD fonksiyonlarında parametre olarak, query_as dönüş tipi vb. için).
use crate::models::fridge_item::FridgeItem;

// Uygulama ömrü boyunca tek bir SqlitePool saklamak için bir OnceCell oluşturuyoruz.
static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

/// Veritabanını başlatıyor. Tabloyu oluşturuyor. Havuzu DB_POOL’a set ediyoruz.
pub async fn init_db() -> Result<(), sqlx::Error> {
    // sqlx ile nutrition.db dosyasına bağlanıyor (yoksa oluşturuyor).
    let pool = SqlitePool::connect("sqlite://nutrition.db").await?;

    // “Tablo var mı? Yoksa oluştur.” SQL komutu.
    query(r#"
        CREATE TABLE IF NOT EXISTS fridge_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            image_url TEXT NOT NULL,
            title TEXT NOT NULL
        )
    "#)
    .execute(&pool)
    .await?;

    // DB_POOL isminde global OnceCell değişkenimize “pool”u atar.
    DB_POOL
        .set(pool)
        .map_err(|_| sqlx::Error::PoolTimedOut)?;

    Ok(())
}

/// OnceCell içindeki SqlitePool’a erişmek için ufak bir yardımcı fonksiyon.
fn get_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Veritabanı henüz başlatılmadı!")
}

// -----------------------------
// CRUD Fonksiyonları
// -----------------------------

/// CREATE
pub async fn add_fridge_item(item: FridgeItem) -> Result<(), sqlx::Error> {
    let pool = get_pool();
    query(r#"INSERT INTO fridge_items (image_url, title) VALUES (?, ?)"#)
        .bind(item.image_url)
        .bind(item.title)
        .execute(pool)
        .await?;
    Ok(())
}

/// READ
pub async fn get_all_fridge_items() -> Result<Vec<FridgeItem>, sqlx::Error> {
    let pool = get_pool();
    let rows = query_as::<_, FridgeItem>("SELECT id, image_url, title FROM fridge_items")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// UPDATE
pub async fn update_fridge_item(id: i64, item: FridgeItem) -> Result<(), sqlx::Error> {
    let pool = get_pool();
    query(r#"UPDATE fridge_items 
            SET image_url = ?, title = ? 
            WHERE id = ?"#)
        .bind(item.image_url)
        .bind(item.title)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// DELETE
pub async fn delete_fridge_item(id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool();
    query(r#"DELETE FROM fridge_items WHERE id = ?"#)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
