use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::infra::error::AppError; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub id: i32, 
    pub name: String,
    pub country_id: i32, 
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>, 
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>, 
}

impl Operator {
    pub fn new(name: String, country_id: i32, created_by: Option<String>) -> Self {
        Self {
            id: 0, // Placeholder; actual ID will be set by the database
            name,
            country_id,
            created_at: Utc::now(),
            created_by,
            updated_at: None,
            updated_by: None,
        }
    }

    pub fn update(&mut self, name: Option<String>, country_id: Option<i32>, updated_by: String) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(country_id) = country_id {
            self.country_id = country_id;
        }
        self.updated_at = Some(Utc::now());
        self.updated_by = Some(updated_by);
    }
}


#[async_trait]
pub trait OperatorRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<Operator>, AppError>;
    async fn find_by_country_id(&self, country_id: i32) -> Result<Vec<Operator>, AppError>;
    async fn find_all(&self) -> Result<Vec<Operator>, AppError>;
    async fn insert(&self, operator: &Operator) -> Result<i32, AppError>; 
    async fn update(&self, operator: &Operator) -> Result<(), AppError>; 
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}