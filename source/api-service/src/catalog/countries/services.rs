use crate::catalog::countries::models::{Country, CountryResponse, NewCountry, UpdateCountry};
use crate::catalog::countries::repositories::CountryRepository;
use crate::core::errors::AppError;
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

    /// Get country by name
    pub async fn get_by_name(pool: &Pool, name: &str) -> Result<Option<Country>, AppError> {
        CountryRepository::get_by_name(pool, name).await
    }

    /// Create a new country
    pub async fn create(pool: &Pool, new_country: NewCountry) -> Result<Country, AppError> {
        CountryRepository::create(pool, new_country).await
    }

    /// Update a country
    pub async fn update(pool: &Pool, id: i32, data: UpdateCountry) -> Result<Country, AppError> {
        CountryRepository::update(pool, id, data).await
    }

    /// Delete a country
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        CountryRepository::delete(pool, id).await
    }

    /// Convert Country into response DTO
    pub fn to_response(country: Country) -> CountryResponse {
        CountryResponse::from(country)
    }

    /// Convert a vector of Country into response DTOs
    pub fn to_response_vec(countries: Vec<Country>) -> Vec<CountryResponse> {
        countries.into_iter().map(CountryResponse::from).collect()
    }
}
