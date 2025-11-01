use crate::core::errors::AppError;
use crate::settings::models::{
    Country, CreateCountry, CreateNetwork, CreateOperator, CreateSorPlan, Network, Operator,
    SorPlan, UpdateCountry, UpdateNetwork, UpdateOperator, UpdateSorPlan,
    Prefix, CreatePrefix, UpdatePrefix,
};
use crate::settings::repositories::{
    CountryRepository, NetworkRepository, OperatorRepository, SorPlanRepository, PrefixRepository,
};
use sqlx::PgPool;

pub struct PrefixService;
pub struct CountryService;
pub struct OperatorService;
pub struct SorPlanService;

impl CountryService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Country>, AppError> {
        CountryRepository::get_all(pool).await
    }

    pub async fn get_by_id(pool: &PgPool, country_id: i32) -> Result<Country, AppError> {
        CountryRepository::get_by_id(pool, country_id)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "Country id {} not found",
                country_id
            )))
    }

    pub async fn create(pool: &PgPool, data: CreateCountry) -> Result<Country, AppError> {
        CountryRepository::create(pool, data).await
    }

    pub async fn update(
        pool: &PgPool,
        country_id: i32,
        data: UpdateCountry,
    ) -> Result<Country, AppError> {
        CountryRepository::update(pool, country_id, data)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "Country id {} not found",
                country_id
            )))
    }

    pub async fn delete(pool: &PgPool, country_id: i32) -> Result<u64, AppError> {
        CountryRepository::delete(pool, country_id).await
    }
}

// =========================
// Operator Service
// =========================

impl OperatorService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Operator>, AppError> {
        OperatorRepository::get_all(pool).await
    }

    pub async fn get_by_id(pool: &PgPool, operator_id: i32) -> Result<Operator, AppError> {
        OperatorRepository::get_by_id(pool, operator_id)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "Operator id {} not found",
                operator_id
            )))
    }

    pub async fn create(pool: &PgPool, data: CreateOperator) -> Result<Operator, AppError> {
        OperatorRepository::create(pool, data).await
    }

    pub async fn update(
        pool: &PgPool,
        operator_id: i32,
        data: UpdateOperator,
    ) -> Result<Operator, AppError> {
        OperatorRepository::update(pool, operator_id, data)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "Operator id {} not found",
                operator_id
            )))
    }

    pub async fn delete(pool: &PgPool, operator_id: i32) -> Result<u64, AppError> {
        OperatorRepository::delete(pool, operator_id).await
    }
}

// =========================
// Network Service
// =========================
pub struct NetworkService;

impl NetworkService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Network>, AppError> {
        NetworkRepository::get_all(pool).await
    }

    pub async fn get_by_id(pool: &PgPool, network_id: i32) -> Result<Network, AppError> {
        NetworkRepository::get_by_id(pool, network_id)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "Network id {} not found",
                network_id
            )))
    }

    pub async fn create(pool: &PgPool, data: CreateNetwork) -> Result<Network, AppError> {
        NetworkRepository::create(pool, data).await
    }

    pub async fn update(
        pool: &PgPool,
        network_id: i32,
        data: UpdateNetwork,
    ) -> Result<Network, AppError> {
        NetworkRepository::update(pool, network_id, data)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "Network id {} not found",
                network_id
            )))
    }

    pub async fn delete(pool: &PgPool, network_id: i32) -> Result<u64, AppError> {
        NetworkRepository::delete(pool, network_id).await
    }
}

impl SorPlanService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<SorPlan>, AppError> {
        SorPlanRepository::get_all(pool).await
    }

    pub async fn get_by_id(pool: &PgPool, sor_plan_id: i32) -> Result<SorPlan, AppError> {
        SorPlanRepository::get_by_id(pool, sor_plan_id)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "SOR Plan id {} not found",
                sor_plan_id
            )))
    }

    pub async fn create(pool: &PgPool, data: CreateSorPlan) -> Result<SorPlan, AppError> {
        SorPlanRepository::create(pool, data).await
    }

    pub async fn update(
        pool: &PgPool,
        sor_plan_id: i32,
        data: UpdateSorPlan,
    ) -> Result<SorPlan, AppError> {
        SorPlanRepository::update(pool, sor_plan_id, data)
            .await?
            .ok_or(AppError::BadRequest(format!(
                "SOR Plan id {} not found",
                sor_plan_id
            )))
    }

    pub async fn delete(pool: &PgPool, sor_plan_id: i32) -> Result<u64, AppError> {
        SorPlanRepository::delete(pool, sor_plan_id).await
    }
}



impl PrefixService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Prefix>, AppError> {
        PrefixRepository::get_all(pool).await
    }

    pub async fn get_by_id(pool: &PgPool, prefix_id: i32) -> Result<Prefix, AppError> {
        // Repository should return Result<Prefix, AppError>
        PrefixRepository::get_by_id(pool, prefix_id).await
    }

    pub async fn create(pool: &PgPool, data: CreatePrefix) -> Result<Prefix, AppError> {
        PrefixRepository::create(pool, data).await
    }

    pub async fn update(
        pool: &PgPool,
        prefix_id: i32,
        data: UpdatePrefix,
    ) -> Result<Prefix, AppError> {
        PrefixRepository::update(pool, prefix_id, data).await
    }

    pub async fn delete(pool: &PgPool, prefix_id: i32) -> Result<u64, AppError> {
        PrefixRepository::delete(pool, prefix_id).await
    }
}
