use crate::core::errors::AppError;
use crate::settings::country_model::{Country, NewCountry, UpdateCountry};
use crate::settings::country_repo::CountryRepository;
use deadpool_postgres::Pool;

pub struct CountryService;

impl CountryService {
    /// Create a new country
    pub async fn create(pool: &Pool, input: NewCountry) -> Result<Country, AppError> {
        CountryRepository::create(pool, input).await
    }

    /// Get all countries
    pub async fn get_all(pool: &Pool) -> Result<Vec<Country>, AppError> {
        CountryRepository::get_all(pool).await
    }

    /// Get a country by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Country, AppError> {
        let country = CountryRepository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Country not found".into()))?;

        Ok(country)
    }

    /// Update a country
    pub async fn update(pool: &Pool, id: i32, data: UpdateCountry) -> Result<Country, AppError> {
        CountryRepository::update(pool, id, data).await
    }

    /// Delete a country
    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        CountryRepository::delete(pool, id).await
    }
}
