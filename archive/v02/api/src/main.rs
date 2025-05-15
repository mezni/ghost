use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
    email: String,
}

async fn create_user(pool: web::Data<Pool>, user: web::Json<NewUser>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let id = Uuid::new_v4();
    let stmt = client
        .prepare("INSERT INTO users (id, name, email) VALUES ($1, $2, $3)")
        .await
        .unwrap();
    client
        .execute(&stmt, &[&id, &user.name, &user.email])
        .await
        .unwrap();
    HttpResponse::Created().json(User {
        id,
        name: user.name.clone(),
        email: user.email.clone(),
    })
}

async fn get_user(pool: web::Data<Pool>, id: web::Path<Uuid>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let stmt = client
        .prepare("SELECT id, name, email FROM users WHERE id = $1")
        .await
        .unwrap();

    if let Some(row) = client.query_opt(&stmt, &[&*id]).await.unwrap() {
        let user = User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        };
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

async fn list_users(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let rows = client
        .query("SELECT id, name, email FROM users", &[])
        .await
        .unwrap();
    let users: Vec<User> = rows
        .iter()
        .map(|row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        })
        .collect();
    HttpResponse::Ok().json(users)
}

async fn delete_user(pool: web::Data<Pool>, id: web::Path<Uuid>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let stmt = client
        .prepare("DELETE FROM users WHERE id = $1")
        .await
        .unwrap();
    let result = client.execute(&stmt, &[&*id]).await.unwrap();
    if result == 1 {
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let mut cfg = Config::new();
    cfg.dbname = Some(std::env::var("DB_NAME").unwrap());
    cfg.user = Some(std::env::var("DB_USER").unwrap());
    cfg.password = Some(std::env::var("DB_PASSWORD").unwrap());
    cfg.host = Some(std::env::var("DB_HOST").unwrap());
    
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).unwrap();


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(list_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
