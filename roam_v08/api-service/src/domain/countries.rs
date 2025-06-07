// This file will contain Country, CountryRepository, and CountryService

use crate::errors::AppError; // Ensure AppError is accessible from this file's location
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc; // For Arc<dyn CountryRepository>

// --- Country Model ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
}

impl Country {
    pub fn new(name: String, code: String, created_by: Option<String>) -> Self {
        Self {
            id: 0, // Placeholder; actual ID will be set by the database
            name,
            code,
            created_at: Utc::now(),
            created_by,
            updated_at: None,
            updated_by: None,
        }
    }

    pub fn update(&mut self, name: Option<String>, code: Option<String>, updated_by: String) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(code) = code {
            self.code = code;
        }
        self.updated_at = Some(Utc::now());
        self.updated_by = Some(updated_by);
    }
}

// --- Country Repository Trait ---
#[async_trait]
pub trait CountryRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<Country>, AppError>;
    async fn find_all(&self) -> Result<Vec<Country>, AppError>;
    async fn insert(&self, country: &Country) -> Result<i32, AppError>;
    async fn update(&self, country: &Country) -> Result<(), AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}

// --- Country Service ---
/// The CountryService encapsulates business logic related to Country entities.
/// It interacts with the CountryRepository to perform data operations.
pub struct CountryService {
    country_repository: Arc<dyn CountryRepository>,
}

impl CountryService {
    /// Creates a new CountryService instance.
    ///
    /// # Arguments
    /// * `country_repository` - An Arc to a type that implements the CountryRepository trait.
    pub fn new(country_repository: Arc<dyn CountryRepository>) -> Self {
        Self { country_repository }
    }

    /// Retrieves a country by its ID.
    pub async fn get_country_by_id(&self, id: i32) -> Result<Option<Country>, AppError> {
        self.country_repository.find_by_id(id).await
    }

    /// Retrieves all countries.
    pub async fn get_all_countries(&self) -> Result<Vec<Country>, AppError> {
        self.country_repository.find_all().await
    }

    /// Creates a new country.
    ///
    /// # Arguments
    /// * `name` - The name of the country.
    /// * `code` - The two-letter ISO code of the country.
    /// * `created_by` - Optional username of the creator.
    pub async fn create_country(
        &self,
        name: String,
        code: String,
        created_by: Option<String>,
    ) -> Result<Country, AppError> {
        // --- Business Logic / Validation goes here ---
        if name.trim().is_empty() {
            return Err(AppError::ServiceError(
                "Country name cannot be empty.".to_string(),
            ));
        }
        if code.trim().len() != 2 {
            return Err(AppError::ServiceError(
                "Country code must be exactly 2 characters long.".to_string(),
            ));
        }

        let new_country = Country::new(name, code.to_uppercase(), created_by);
        let id = self.country_repository.insert(&new_country).await?;

        // After insertion, fetch the country to get its full state (including generated ID, created_at)
        // This is a common pattern to ensure the returned object is complete and matches DB state.
        self.country_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| {
                AppError::ServiceError(format!(
                    "Failed to retrieve created country with ID: {}",
                    id
                ))
            })
    }

    /// Updates an existing country.
    ///
    /// # Arguments
    /// * `id` - The ID of the country to update.
    /// * `name` - Optional new name.
    /// * `code` - Optional new code.
    /// * `updated_by` - Username of the updater.
    pub async fn update_country(
        &self,
        id: i32,
        name: Option<String>,
        code: Option<String>,
        updated_by: String,
    ) -> Result<Country, AppError> {
        let mut existing_country =
            self.country_repository
                .find_by_id(id)
                .await?
                .ok_or_else(|| {
                    AppError::ServiceError(format!("Country with ID {} not found for update.", id))
                })?;

        // Apply business logic / validation for updates
        if let Some(ref n) = name {
            if n.trim().is_empty() {
                return Err(AppError::ServiceError(
                    "Country name cannot be empty.".to_string(),
                ));
            }
        }
        if let Some(ref c) = code {
            if c.trim().len() != 2 {
                return Err(AppError::ServiceError(
                    "Country code must be exactly 2 characters long.".to_string(),
                ));
            }
        }

        existing_country.update(name, code.map(|c| c.to_uppercase()), updated_by);
        self.country_repository.update(&existing_country).await?;

        // Fetch the updated country to return its current state
        Ok(existing_country)
    }

    /// Deletes a country by its ID.
    pub async fn delete_country(&self, id: i32) -> Result<(), AppError> {
        self.country_repository.delete(id).await
    }
}
