use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::infra::error::AppError; 


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

#[async_trait]
pub trait CountryRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<Country>, AppError>;
    async fn find_all(&self) -> Result<Vec<Country>, AppError>;
    async fn insert(&self, country: &Country) -> Result<i32, AppError>; 
    async fn update(&self, country: &Country) -> Result<(), AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}