use deadpool_postgres::Client;
use crate::dtos::*;
use crate::errors::AppError;

pub async fn insert_country(client: &Client, dto: &CreateCountryDto) -> Result<CountryDto, AppError> {
    let stmt = client.prepare("INSERT INTO countries (country_name, iso) VALUES ($1, $2) RETURNING id").await?;
    let row = client.query_one(&stmt, &[&dto.country_name, &dto.iso]).await?;
    Ok(CountryDto { id: row.get(0), country_name: dto.country_name.clone(), iso: dto.iso.clone() })
}

pub async fn get_countries(client: &Client) -> Result<Vec<CountryDto>, AppError> {
    let stmt = client.prepare("SELECT id, country_name, iso FROM countries").await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows.into_iter().map(|r| CountryDto {
        id: r.get(0),
        country_name: r.get(1),
        iso: r.get(2),
    }).collect())
}

pub async fn insert_operator(client: &Client, dto: &CreateOperatorDto) -> Result<OperatorDto, AppError> {
    let stmt = client.prepare("INSERT INTO operators (operator_name, country_id) VALUES ($1, $2) RETURNING id").await?;
    let row = client.query_one(&stmt, &[&dto.operator_name, &dto.country_id]).await?;
    Ok(OperatorDto { id: row.get(0), operator_name: dto.operator_name.clone(), country_id: dto.country_id })
}

pub async fn get_operators(client: &Client) -> Result<Vec<OperatorDto>, AppError> {
    let stmt = client.prepare("SELECT id, operator_name, country_id FROM operators").await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows.into_iter().map(|r| OperatorDto {
        id: r.get(0),
        operator_name: r.get(1),
        country_id: r.get(2),
    }).collect())
}

pub async fn insert_plan(client: &Client, dto: &CreatePlanDto) -> Result<PlanDto, AppError> {
    if !(0.0..=100.0).contains(&dto.percentage) {
        return Err(AppError::ValidationError("Percentage must be between 0 and 100".into()));
    }
    let stmt = client.prepare("INSERT INTO plans (country_id, operator_id, percentage) VALUES ($1, $2, $3) RETURNING id").await?;
    let row = client.query_one(&stmt, &[&dto.country_id, &dto.operator_id, &dto.percentage]).await?;
    Ok(PlanDto {
        id: row.get(0),
        country_id: dto.country_id,
        operator_id: dto.operator_id,
        percentage: dto.percentage,
    })
}

pub async fn get_plans(client: &Client) -> Result<Vec<PlanDto>, AppError> {
    let stmt = client.prepare("SELECT id, country_id, operator_id, percentage FROM plans").await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows.into_iter().map(|r| PlanDto {
        id: r.get(0),
        country_id: r.get(1),
        operator_id: r.get(2),
        percentage: r.get(3),
    }).collect())
}
