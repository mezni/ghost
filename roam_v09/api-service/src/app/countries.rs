use crate::domain::countries::{Country, CountryRepository};
use chrono::Utc;
use std::sync::Arc;

/// Application service responsible for coordinating domain and infrastructure for countries.
pub struct CountryService<R: CountryRepository + Send + Sync> {
    repo: Arc<R>,
}

impl<R: CountryRepository + Send + Sync> CountryService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    fn capitalize_name(name: &str) -> String {
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    pub async fn create_country(
        &self,
        name: String,
        code: String,
        created_by: String,
    ) -> Result<Country, String> {
        let formatted_name = Self::capitalize_name(&name);

        let country = Country::builder()
            .name(formatted_name)
            .code(code)
            .created_by(created_by.clone())
            .created_at(Utc::now())
            .build();

        self.repo.insert(country).await
    }

    pub async fn update_country(
        &self,
        id: i32,
        name: Option<String>,
        code: Option<String>,
        updated_by: String,
    ) -> Result<(), String> {
        let existing = self.repo.get_by_id(id).await?;
        if let Some(mut country) = existing {
            if let Some(n) = name {
                country.name = Self::capitalize_name(&n);
            }
            if let Some(c) = code {
                country.code = c;
            }
            country.updated_by = Some(updated_by);
            country.updated_at = Some(Utc::now());
            self.repo.update(country).await
        } else {
            Err("Country not found".into())
        }
    }

    pub async fn delete_country(&self, id: i32) -> Result<(), String> {
        self.repo.delete(id).await
    }

    pub async fn get_country(&self, id: i32) -> Result<Option<Country>, String> {
        self.repo.get_by_id(id).await
    }

    pub async fn list_countries(&self) -> Result<Vec<Country>, String> {
        self.repo.list().await
    }
}
