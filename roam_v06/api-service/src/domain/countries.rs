use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::errors::AppError;

/// Represents a country with an ID, name, and ISO alpha-2 code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String, // ISO alpha-2 code (e.g. "US", "FR")
}

impl Country {
    /// Creates a new Country instance with name capitalized and code uppercased.
    pub fn new(id: i32, name: String, code: String) -> Self {
        let formatted_name = name
            .trim()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();

        let formatted_code = code.trim().to_uppercase();

        Self {
            id,
            name: formatted_name,
            code: formatted_code,
        }
    }
}

/// Repository trait for managing countries in persistent storage.
#[async_trait]
pub trait CountryRepository {
    async fn get_all(&self) -> Result<Vec<Country>, AppError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Country>, AppError>;
    async fn create(&self, country: Country) -> Result<Country, AppError>;
    async fn update(&self, country: Country) -> Result<(), AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}
