use crate::core::errors::AppError;
use crate::settings::models::{
    Country, CreateCountry, CreateNetwork, CreateOperator, CreatePrefix, CreateSorPlan, Network,
    Operator, Prefix, SorPlan, UpdateCountry, UpdateNetwork, UpdateOperator, UpdatePrefix,
    UpdateSorPlan,
};
use crate::settings::services::{
    CountryService, NetworkService, OperatorService, PrefixService, SorPlanService,
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    // =========================
    // Countries
    // =========================
    cfg.service(
        web::scope("/settings/countries")
            .route("", web::get().to(get_all_countries))
            .route("/{id}", web::get().to(get_country_by_id))
            .route("", web::post().to(create_country))
            .route("/{id}", web::put().to(update_country))
            .route("/{id}", web::delete().to(delete_country)),
    );

    // =========================
    // Operators
    // =========================
    cfg.service(
        web::scope("/settings/operators")
            .route("", web::get().to(get_all_operators))
            .route("/{id}", web::get().to(get_operator_by_id))
            .route(
                "/country/{country_id}",
                web::get().to(get_operators_by_country),
            ) // <-- Added this line
            .route("", web::post().to(create_operator))
            .route("/{id}", web::put().to(update_operator))
            .route("/{id}", web::delete().to(delete_operator)),
    );

    // =========================
    // Networks
    // =========================
    cfg.service(
        web::scope("/settings/networks")
            .route("", web::get().to(get_all_networks))
            .route("/{id}", web::get().to(get_network_by_id))
            .route("", web::post().to(create_network))
            .route("/{id}", web::put().to(update_network))
            .route("/{id}", web::delete().to(delete_network)),
    );

    // =========================
    // Prefixes
    // =========================
    cfg.service(
        web::scope("/settings/prefixes")
            .route("", web::get().to(get_all_prefixes))
            .route("/{id}", web::get().to(get_prefix_by_id))
            .route("", web::post().to(create_prefix))
            .route("/{id}", web::put().to(update_prefix))
            .route("/{id}", web::delete().to(delete_prefix)),
    );

    // =========================
    // SOR Plan
    // =========================
    cfg.service(
        web::scope("/settings/sor_plan")
            .route("", web::get().to(get_all_sor_plans))
            .route("/{id}", web::get().to(get_sor_plan_by_id))
            .route("", web::post().to(create_sor_plan))
            .route("/{id}", web::put().to(update_sor_plan))
            .route("/{id}", web::delete().to(delete_sor_plan)),
    );
}

// =========================
// Country Handlers
// =========================
async fn get_all_countries(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let countries = CountryService::get_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(countries))
}

async fn get_country_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let country = CountryService::get_by_id(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(country))
}

async fn create_country(
    pool: web::Data<PgPool>,
    body: web::Json<CreateCountry>,
) -> Result<HttpResponse, AppError> {
    let country = CountryService::create(pool.get_ref(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(country))
}

async fn update_country(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateCountry>,
) -> Result<HttpResponse, AppError> {
    let country =
        CountryService::update(pool.get_ref(), path.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(country))
}

async fn delete_country(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let deleted = CountryService::delete(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

// =========================
// Operator Handlers
// =========================
async fn get_all_operators(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let operators = OperatorService::get_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(operators))
}

async fn get_operator_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let operator = OperatorService::get_by_id(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(operator))
}

// NEW: Get operators by country ID
async fn get_operators_by_country(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let country_id = path.into_inner();
    let operators = OperatorService::get_by_country_id(pool.get_ref(), country_id).await?;
    Ok(HttpResponse::Ok().json(operators))
}

async fn create_operator(
    pool: web::Data<PgPool>,
    body: web::Json<CreateOperator>,
) -> Result<HttpResponse, AppError> {
    let operator = OperatorService::create(pool.get_ref(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(operator))
}

async fn update_operator(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateOperator>,
) -> Result<HttpResponse, AppError> {
    let operator =
        OperatorService::update(pool.get_ref(), path.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(operator))
}

async fn delete_operator(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let deleted = OperatorService::delete(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

// =========================
// Network Handlers
// =========================
async fn get_all_networks(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let networks = NetworkService::get_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(networks))
}

async fn get_network_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let network = NetworkService::get_by_id(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(network))
}

async fn create_network(
    pool: web::Data<PgPool>,
    body: web::Json<CreateNetwork>,
) -> Result<HttpResponse, AppError> {
    let network = NetworkService::create(pool.get_ref(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(network))
}

async fn update_network(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateNetwork>,
) -> Result<HttpResponse, AppError> {
    let network =
        NetworkService::update(pool.get_ref(), path.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(network))
}

async fn delete_network(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let deleted = NetworkService::delete(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

// SOR Plan handlers
async fn get_all_sor_plans(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let plans = SorPlanService::get_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(plans))
}

async fn get_sor_plan_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let plan = SorPlanService::get_by_id(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(plan))
}

async fn create_sor_plan(
    pool: web::Data<PgPool>,
    body: web::Json<CreateSorPlan>,
) -> Result<HttpResponse, AppError> {
    let plan = SorPlanService::create(pool.get_ref(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(plan))
}

async fn update_sor_plan(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateSorPlan>,
) -> Result<HttpResponse, AppError> {
    let plan = SorPlanService::update(pool.get_ref(), path.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(plan))
}

async fn delete_sor_plan(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let deleted = SorPlanService::delete(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

// =========================
// Prefix Handlers
// =========================
async fn get_all_prefixes(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let prefixes = PrefixService::get_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(prefixes))
}

async fn get_prefix_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let prefix = PrefixService::get_by_id(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(prefix))
}

async fn create_prefix(
    pool: web::Data<PgPool>,
    body: web::Json<CreatePrefix>,
) -> Result<HttpResponse, AppError> {
    let prefix = PrefixService::create(pool.get_ref(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(prefix))
}

async fn update_prefix(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdatePrefix>,
) -> Result<HttpResponse, AppError> {
    let prefix =
        PrefixService::update(pool.get_ref(), path.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(prefix))
}

async fn delete_prefix(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let deleted = PrefixService::delete(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(deleted))
}
