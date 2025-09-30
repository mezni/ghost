use crate::catalog::sor::models::{NewSorPlan, SorPlan, SorPlanResponse, UpdateSorPlan};
use crate::catalog::sor::repositories::SorPlanRepository;
use crate::core::errors::AppError;
use deadpool_postgres::Pool;

pub struct SorPlanService;

impl SorPlanService {
    /// Create a new SOR plan
    pub async fn create(pool: &Pool, new_plan: NewSorPlan) -> Result<SorPlan, AppError> {
        SorPlanRepository::create(pool, new_plan).await
    }

    /// Get all SOR plans
    pub async fn get_all(pool: &Pool) -> Result<Vec<SorPlan>, AppError> {
        SorPlanRepository::get_all(pool).await
    }

    /// Get a SOR plan by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<SorPlan>, AppError> {
        SorPlanRepository::get_by_id(pool, id).await
    }

    /// Update a SOR plan (soft-delete old, insert new with incremented version)
    pub async fn update(pool: &Pool, id: i32, data: UpdateSorPlan) -> Result<SorPlan, AppError> {
        // routage_type_id is None, repository will handle it
        SorPlanRepository::update(pool, id, data, None).await
    }

    /// Soft-delete a SOR plan
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        SorPlanRepository::delete(pool, id).await
    }

    /// Convert SorPlan into response DTO
    pub fn to_response(plan: SorPlan) -> SorPlanResponse {
        SorPlanResponse::from(plan)
    }

    /// Convert a vector of SorPlan into response DTOs
    pub fn to_response_vec(plans: Vec<SorPlan>) -> Vec<SorPlanResponse> {
        plans.into_iter().map(SorPlanResponse::from).collect()
    }
}
