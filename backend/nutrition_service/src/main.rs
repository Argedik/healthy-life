use actix_web::{get, post, delete, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv; // .env dosyasını yüklemek için

// Veritabanı tablomuzdaki kaydı temsil eden model.
#[derive(Serialize, Deserialize)]
struct FridgeItem {
    id: i32,
    title: String,
    image_url: String,
}

// GET /fridge_items: Tüm buzdolabı kayıtlarını veritabanından çekip JSON olarak döner.
#[get("/fridge_items")]
async fn get_fridge_items(db_pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query_as!(FridgeItem, r#"
        SELECT id, title, image_url
        FROM fridge_items
        ORDER BY id
    "#)
    .fetch_all(&**db_pool)
    .await;

    match rows {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            eprintln!("DB Error: {:?}", e);
            HttpResponse::InternalServerError().body("Veritabanı hatası")
        }
    }
}

// POST /fridge_items: Yeni bir buzdolabı kaydı ekler.
#[derive(Deserialize)]
struct CreateItem {
    title: String,
    image_url: String,
}

#[post("/fridge_items")]
async fn create_fridge_item(
    db_pool: web::Data<PgPool>,
    item: web::Json<CreateItem>
) -> impl Responder {
    let result = sqlx::query_as!(
        FridgeItem,
        r#"
        INSERT INTO fridge_items (title, image_url)
        VALUES ($1, $2)
        RETURNING id, title, image_url
        "#,
        item.title,
        item.image_url
    )
    .fetch_one(&**db_pool)
    .await;

    match result {
        Ok(new_item) => HttpResponse::Ok().json(new_item),
        Err(e) => {
            eprintln!("Insert Error: {:?}", e);
            HttpResponse::InternalServerError().body("Kayıt ekleme hatası")
        }
    }
}

// DELETE /fridge_items/{id}: Belirtilen id'ye sahip kaydı veritabanından siler.
#[delete("/fridge_items/{id}")]
async fn delete_fridge_item(db_pool: web::Data<PgPool>, web::Path(id): web::Path<i32>) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM fridge_items WHERE id = $1",
        id
    )
    .execute(&**db_pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Silme başarılı"),
        Err(e) => {
            eprintln!("Delete Error: {:?}", e);
            HttpResponse::InternalServerError().body("Silme hatası")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ortam değişkenlerini .env dosyasından yükle.
    dotenv().ok();

    // DATABASE_URL ortam değişkenini alıyoruz. Yoksa varsayılanı kullanıyoruz.
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:pass@localhost:5432/besin".to_string());

    // PostgreSQL bağlantı havuzu oluşturuluyor.
    let pool = PgPool::connect(&database_url).await
        .expect("Veritabanına bağlanılamadı!");

    println!("Starting nutrition service on port 8080...");

    // HTTP sunucusunu oluşturuyoruz, CORS ayarlarını yapılandırıyoruz ve endpoint'leri ekliyoruz.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .service(get_fridge_items)
            .service(create_fridge_item)
            .service(delete_fridge_item) // Yeni eklenen delete endpoint
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
