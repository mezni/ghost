use crate::core::errors::AppError;
use crate::settings::models::{Country, CountryDTO, CreateCountry, UpdateCountry};
use crate::settings::repositories::CountryRepository;
use deadpool_postgres::Pool;

pub struct CountryService;

impl CountryService {
    /// Get a country by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<CountryDTO, AppError> {
        let country = CountryRepository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::Other("Country not found".into()))?;
        Ok(CountryDTO::from(country))
    }

    /// Get all countries
    pub async fn get_all(pool: &Pool) -> Result<Vec<CountryDTO>, AppError> {
        let countries = CountryRepository::get_all(pool).await?;
        Ok(countries.into_iter().map(CountryDTO::from).collect())
    }

    /// Create a new country
    pub async fn create(pool: &Pool, input: CreateCountry) -> Result<CountryDTO, AppError> {
        // Prevent duplicates by ISO code
        if let Some(_) = CountryRepository::get_by_iso_code(pool, &input.iso_code).await? {
            return Err(AppError::Other(format!(
                "Country with ISO code {} already exists",
                input.iso_code
            )));
        }

        let country = CountryRepository::create(pool, input).await?;
        Ok(CountryDTO::from(country))
    }

    /// Update an existing country
    pub async fn update(
        pool: &Pool,
        id: i32,
        input: UpdateCountry,
    ) -> Result<CountryDTO, AppError> {
        let country = CountryRepository::update(pool, id, input).await?;
        Ok(CountryDTO::from(country))
    }

    /// Delete a country by ID
    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        CountryRepository::delete(pool, id).await
    }
}
