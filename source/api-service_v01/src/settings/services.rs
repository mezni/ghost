use crate::core::errors::AppError;
use crate::settings::models::{
    CountryDTO, CreateCountry, CreateOperator, OperatorDTO, UpdateCountry, UpdateOperator,
    CreateNetwork, NetworkDTO, UpdateNetwork,
};
use crate::settings::repositories::{CountryRepository, OperatorRepository, NetworkRepository};
use deadpool_postgres::Pool;

// -------------------------
// Country Service
// -------------------------
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

// -------------------------
// Operator Service
// -------------------------
pub struct OperatorService;

impl OperatorService {
    /// Get all operators
    pub async fn get_all(pool: &Pool) -> Result<Vec<OperatorDTO>, AppError> {
        let operators = OperatorRepository::get_all(pool).await?;
        Ok(operators.into_iter().map(OperatorDTO::from).collect())
    }

    /// Get operator by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<OperatorDTO, AppError> {
        let operator = OperatorRepository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::Other("Operator not found".into()))?;
        Ok(OperatorDTO::from(operator))
    }

    /// Create a new operator
    pub async fn create(pool: &Pool, input: CreateOperator) -> Result<OperatorDTO, AppError> {
        let operator = OperatorRepository::create(pool, input).await?;
        Ok(OperatorDTO::from(operator))
    }

    /// Update an existing operator
    pub async fn update(pool: &Pool, input: UpdateOperator) -> Result<OperatorDTO, AppError> {
        let operator = OperatorRepository::update(pool, &input).await?;
        Ok(OperatorDTO::from(operator))
    }

    /// Soft delete an operator
    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        OperatorRepository::delete(pool, id).await
    }
}


// -------------------------
// Network Service
// -------------------------
pub struct NetworkService;

impl NetworkService {
    /// Get all networks
    pub async fn get_all(pool: &Pool) -> Result<Vec<NetworkDTO>, AppError> {
        let networks = NetworkRepository::get_all(pool).await?;
        Ok(networks.into_iter().map(NetworkDTO::from).collect())
    }

    /// Get network by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<NetworkDTO, AppError> {
        let network = NetworkRepository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::Other("Network not found".into()))?;
        Ok(NetworkDTO::from(network))
    }

    /// Create a new network
    pub async fn create(pool: &Pool, input: CreateNetwork) -> Result<NetworkDTO, AppError> {
        let network = NetworkRepository::create(pool, input).await?;
        Ok(NetworkDTO::from(network))
    }

    /// Update an existing network
    pub async fn update(pool: &Pool, input: UpdateNetwork) -> Result<NetworkDTO, AppError> {
        let network = NetworkRepository::update(pool, &input).await?;
        Ok(NetworkDTO::from(network))
    }

    /// Soft delete a network
    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        NetworkRepository::delete(pool, id).await
    }
}
