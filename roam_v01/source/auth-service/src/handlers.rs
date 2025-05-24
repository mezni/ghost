use actix_web::{HttpResponse, Responder, get, web};

#[get("/health")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Auth Service is healthy.")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").service(healthcheck));
}
