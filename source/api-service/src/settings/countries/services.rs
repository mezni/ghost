use crate::core::errors::AppError;
use crate::settings::countries::models::{Country, NewCountry, UpdateCountry};
use crate::settings::countries::repositories::CountryRepository;
use deadpool_postgres::Pool;

pub struct CountryService;

impl CountryService {
    /// Get all countries
    pub async fn get_all(pool: &Pool) -> Result<Vec<Country>, AppError> {
        CountryRepository::get_all(pool).await
    }

    /// Get country by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Country>, AppError> {
        CountryRepository::get_by_id(pool, id).await
    }

    /// Create new country
    pub async fn create(pool: &Pool, new_country: NewCountry) -> Result<Country, AppError> {
        // Validate input
        if new_country.iso_code.trim().is_empty() {
            return Err(AppError::Other("ISO code cannot be empty".into()));
        }

        if new_country.country_name.trim().is_empty() {
            return Err(AppError::Other("Country name cannot be empty".into()));
        }

        // Pass ownership directly
        CountryRepository::create(pool, new_country).await
    }

    /// Update country
    pub async fn update(pool: &Pool, id: i32, update: UpdateCountry) -> Result<Country, AppError> {
        // Validate input
        if let Some(ref name) = update.country_name {
            if name.trim().is_empty() {
                return Err(AppError::Other("Country name cannot be empty".into()));
            }
        }

        // Pass ownership directly (no &)
        CountryRepository::update(pool, id, update).await
    }

    /// Delete country
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        CountryRepository::delete(pool, id).await
    }
}
