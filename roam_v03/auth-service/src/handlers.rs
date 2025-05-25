use actix_web::{HttpResponse, Responder, web};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Auth service is healthy")
}
