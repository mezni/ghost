use crate::settings::models::{Country, CreateCountry, UpdateCountry};
use crate::settings::repositories::CountryRepository;
use crate::core::errors::AppError;
use sqlx::PgPool;

pub struct CountryService;

impl CountryService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Country>, AppError> {
        CountryRepository::get_all(pool).await
    }

    pub async fn get_by_id(pool: &PgPool, country_id: i32) -> Result<Country, AppError> {
        CountryRepository::get_by_id(pool, country_id)
            .await?
            .ok_or(AppError::BadRequest(format!("Country id {} not found", country_id)))
    }

    pub async fn create(pool: &PgPool, data: CreateCountry) -> Result<Country, AppError> {
        CountryRepository::create(pool, data).await
    }

    pub async fn update(pool: &PgPool, country_id: i32, data: UpdateCountry) -> Result<Country, AppError> {
        CountryRepository::update(pool, country_id, data)
            .await?
            .ok_or(AppError::BadRequest(format!("Country id {} not found", country_id)))
    }

    pub async fn delete(pool: &PgPool, country_id: i32) -> Result<u64, AppError> {
        CountryRepository::delete(pool, country_id).await
    }
}
