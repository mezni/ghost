use crate::domain::entities::country::Country;
use crate::domain::repositories::country_repository::CountryRepository;
use crate::errors::AppError;
use async_trait::async_trait;

pub struct CountryService<R: CountryRepository + Send + Sync> {
    repo: R,
}

impl<R: CountryRepository + Send + Sync> CountryService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list_countries(&self) -> Result<Vec<Country>, AppError> {
        self.repo.get_all().await
    }

    pub async fn get_country(&self, id: i32) -> Result<Option<Country>, AppError> {
        self.repo.get_by_id(id).await
    }


}
