// src/settings/operators/services.rs
use crate::core::errors::AppError;
use crate::settings::operators::models::{Operator, NewOperator, UpdateOperator};
use crate::settings::operators::repositories::OperatorRepository;
use deadpool_postgres::Pool;

pub struct OperatorService;

impl OperatorService {
    /// Get all operators
    pub async fn get_all(pool: &Pool) -> Result<Vec<Operator>, AppError> {
        OperatorRepository::get_all(pool).await
    }

    /// Get operator by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Operator>, AppError> {
        OperatorRepository::get_by_id(pool, id).await
    }

    /// Create new operator
    pub async fn create(pool: &Pool, new_op: NewOperator) -> Result<Operator, AppError> {
        // Validate input
        if new_op.operator_name.trim().is_empty() {
            return Err(AppError::Other("Operator name cannot be empty".into()));
        }
        if new_op.country_name.trim().is_empty() {
            return Err(AppError::Other("Country name cannot be empty".into()));
        }

        // Pass ownership directly
        OperatorRepository::create(pool, new_op).await
    }

    /// Update operator
    pub async fn update(pool: &Pool, id: i32, update: UpdateOperator) -> Result<Operator, AppError> {
        // Validate input
        if let Some(ref name) = update.operator_name {
            if name.trim().is_empty() {
                return Err(AppError::Other("Operator name cannot be empty".into()));
            }
        }

        if let Some(ref country) = update.country_name {
            if country.trim().is_empty() {
                return Err(AppError::Other("Country name cannot be empty".into()));
            }
        }

        // Pass ownership directly
        OperatorRepository::update(pool, id, update).await
    }

    /// Delete operator
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        OperatorRepository::delete(pool, id).await
    }
}
