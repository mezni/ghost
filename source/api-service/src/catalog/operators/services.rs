use crate::catalog::operators::models::{NewOperator, Operator, UpdateOperator};
use crate::catalog::operators::repositories::OperatorRepository;
use crate::core::errors::AppError;
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

    /// Get operators by country_id
    pub async fn get_by_country_id(
        pool: &Pool,
        country_id: i32,
    ) -> Result<Vec<Operator>, AppError> {
        OperatorRepository::get_by_country_id(pool, country_id).await
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

        OperatorRepository::create(pool, &new_op).await
    }

    /// Update operator
    pub async fn update(
        pool: &Pool,
        id: i32,
        update: UpdateOperator,
    ) -> Result<Option<Operator>, AppError> {
        if let Some(ref name) = update.operator_name {
            if name.trim().is_empty() {
                return Err(AppError::Other("Operator name cannot be empty".into()));
            }
        }
        if let Some(ref country_name) = update.country_name {
            if country_name.trim().is_empty() {
                return Err(AppError::Other("Country name cannot be empty".into()));
            }
        }

        OperatorRepository::update(pool, id, &update).await
    }

    /// Delete operator (hard delete)
    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        OperatorRepository::delete(pool, id).await
    }
}
