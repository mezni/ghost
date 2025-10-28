use actix_web::{HttpResponse, Responder, get, web};
use sqlx::PgPool;

#[get("/health")]
async fn health_check(db: web::Data<PgPool>) -> impl Responder {
    // Optional: run a quick DB check
    if let Err(e) = sqlx::query("SELECT 1").execute(db.get_ref()).await {
        return HttpResponse::InternalServerError().body(format!("❌ DB connection error: {}", e));
    }

    HttpResponse::Ok().body("✅ Server and DB are up!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
