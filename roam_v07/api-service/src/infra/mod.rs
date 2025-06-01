use crate::domain::countries::{Country, CountryRepository};
use crate::infra::error::AppError;
use serde::{Serialize, Deserialize};
use crate::infra::postgres::countries::PgCountryRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCountryCommand {
    pub name: String,
    pub code: String,
    pub created_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCountryCommand {
    pub id: i32,
    pub name: Option<String>,
    pub code: Option<String>,
    pub updated_by: String,
}

pub struct CountryService<R: CountryRepository> {
    country_repository: R,
}

impl<R: CountryRepository> CountryService<R> {
    pub fn new(country_repository: R) -> Self {
        Self { country_repository }
    }

    pub async fn create_country(&self, cmd: CreateCountryCommand) -> Result<Country, AppError> {
        if cmd.name.trim().is_empty() || cmd.code.trim().is_empty() {
            return Err(AppError::Validation("Name and code cannot be empty.".to_string()));
        }

        let mut country = Country::new(cmd.name, cmd.code, cmd.created_by);
        let generated_id = self.country_repository.insert(&country).await?;
        country.id = generated_id;
        Ok(country)
    }

    pub async fn get_country(&self, id: i32) -> Result<Country, AppError> {
        self.country_repository.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Country with ID {} not found.", id)))
    }

    pub async fn get_all_countries(&self) -> Result<Vec<Country>, AppError> {
        self.country_repository.find_all().await
    }

    pub async fn update_country(&self, cmd: UpdateCountryCommand) -> Result<Country, AppError> {
        let mut country = self.country_repository.find_by_id(cmd.id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Country with ID {} not found for update.", cmd.id)))?;

        country.update(cmd.name, cmd.code, cmd.updated_by);

        self.country_repository.update(&country).await?;
        Ok(country)
    }

    pub async fn delete_country(&self, id: i32) -> Result<(), AppError> {
        self.country_repository.delete(id).await
    }
}