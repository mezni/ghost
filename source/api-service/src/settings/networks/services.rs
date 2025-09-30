use crate::core::errors::AppError;
use crate::settings::networks::models::{Network, NewNetwork, UpdateNetwork};
use crate::settings::networks::repositories::NetworkRepository;
use deadpool_postgres::Pool;

pub struct NetworkService;

impl NetworkService {
    /// Get all networks
    pub async fn get_all(pool: &Pool) -> Result<Vec<Network>, AppError> {
        NetworkRepository::get_all(pool).await
    }

    /// Get network by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Network>, AppError> {
        NetworkRepository::get_by_id(pool, id).await
    }

    /// Create new network
    pub async fn create(pool: &Pool, new_network: NewNetwork) -> Result<Network, AppError> {
        // Validate input
        if new_network.operator_name.trim().is_empty() {
            return Err(AppError::Other("Operator name cannot be empty".into()));
        }
        if new_network.country_name.trim().is_empty() {
            return Err(AppError::Other("Country name cannot be empty".into()));
        }
        if new_network.plmn_code.trim().is_empty() {
            return Err(AppError::Other("PLMN code cannot be empty".into()));
        }

        NetworkRepository::create(pool, new_network).await
    }

    /// Update network
    pub async fn update(pool: &Pool, id: i32, update: UpdateNetwork) -> Result<Network, AppError> {
        // Validate input
        if let Some(ref op_name) = update.operator_name {
            if op_name.trim().is_empty() {
                return Err(AppError::Other("Operator name cannot be empty".into()));
            }
        }
        if let Some(ref ct_name) = update.country_name {
            if ct_name.trim().is_empty() {
                return Err(AppError::Other("Country name cannot be empty".into()));
            }
        }

        NetworkRepository::update(pool, id, update).await
    }

    /// Delete network
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        NetworkRepository::delete(pool, id).await
    }
}
