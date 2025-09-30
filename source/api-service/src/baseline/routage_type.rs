use crate::core::errors::AppError;
use actix_web::{HttpResponse, Scope, web};
use chrono::NaiveDateTime;
use chrono::Utc;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoutageType {
    pub routage_type_id: i32,
    pub routage_type_name: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

/// Response DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct RoutageTypeResponse {
    pub routage_type_id: i32,
    pub routage_type_name: String,
}

impl From<RoutageType> for RoutageTypeResponse {
    fn from(routage: RoutageType) -> Self {
        RoutageTypeResponse {
            routage_type_id: routage.routage_type_id,
            routage_type_name: routage.routage_type_name, // fix: was `country.routage_type_name`
        }
    }
}

pub struct RoutageTypeRepository;

impl RoutageTypeRepository {
    pub async fn get_all(pool: &Pool) -> Result<Vec<RoutageType>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"SELECT * FROM dim_routage_types ORDER BY routage_type_name"#;

        let rows = client.query(stmt, &[]).await.map_err(AppError::Db)?;
        Ok(rows
            .into_iter()
            .map(|row| RoutageType {
                routage_type_id: row.get("routage_type_id"), // fix: was country_id
                routage_type_name: row.get("routage_type_name"), // fix: was country_name
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect())
    }

    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<RoutageType>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "SELECT * FROM dim_routage_types WHERE routage_type_id = $1";

        if let Some(row) = client.query_opt(stmt, &[&id]).await.map_err(AppError::Db)? {
            Ok(Some(RoutageType {
                routage_type_id: row.get("routage_type_id"),
                routage_type_name: row.get("routage_type_name"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct RoutageTypeService;

impl RoutageTypeService {
    /// Get all routage types
    pub async fn get_all(pool: &Pool) -> Result<Vec<RoutageType>, AppError> {
        RoutageTypeRepository::get_all(pool).await
    }

    /// Get routage type by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<RoutageType>, AppError> {
        RoutageTypeRepository::get_by_id(pool, id).await
    }

    /// Convert RoutageType into response DTO
    pub fn to_response(routage: RoutageType) -> RoutageTypeResponse {
        RoutageTypeResponse::from(routage)
    }

    /// Convert a vector of RoutageType into response DTOs
    pub fn to_response_vec(routages: Vec<RoutageType>) -> Vec<RoutageTypeResponse> {
        routages
            .into_iter()
            .map(RoutageTypeResponse::from)
            .collect()
    }
}

pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let routages = RoutageTypeService::get_all(&pool).await?;
    let resp: Vec<RoutageTypeResponse> = RoutageTypeService::to_response_vec(routages);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_by_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if let Some(routage) = RoutageTypeService::get_by_id(&pool, id.into_inner()).await? {
        let resp: RoutageTypeResponse = RoutageTypeService::to_response(routage);
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body("Routage type not found"))
    }
}

pub fn scope() -> Scope {
    web::scope("/routagetypes")
        .route("", web::get().to(get_all))
        .route("/{id}", web::get().to(get_by_id))
}
