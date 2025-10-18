// services/lookup.rs
use crate::core::errors::AppError;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Prefixes {
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

pub struct PrefixLookup {
    prefix_map: HashMap<String, (Option<i32>, Option<i32>)>,
}

impl PrefixLookup {
    pub async fn new(pool: &Pool<Postgres>) -> Result<Self, AppError> {
        let prefix_map = Self::load_prefixes_from_db(pool).await?;
        Ok(PrefixLookup { prefix_map })
    }

    async fn load_prefixes_from_db(
        pool: &Pool<Postgres>,
    ) -> Result<HashMap<String, (Option<i32>, Option<i32>)>, AppError> {
        // Option 1: Use query macro (requires DATABASE_URL set)
        let prefixes = sqlx::query!(
            r#"
            SELECT prefix, country_id, operator_id 
            FROM cfg_prefixes 
            WHERE is_valid IS TRUE
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut prefix_map = HashMap::with_capacity(prefixes.len());
        
        for record in prefixes {
            prefix_map.insert(
                record.prefix,
                (record.country_id, record.operator_id),
            );
        }

        Ok(prefix_map)
    }

    // Alternative method without query macro
    async fn load_prefixes_from_db_manual(
        pool: &Pool<Postgres>,
    ) -> Result<HashMap<String, (Option<i32>, Option<i32>)>, AppError> {
        use sqlx::Row;
        
        let rows = sqlx::query(
            r#"
            SELECT prefix, country_id, operator_id 
            FROM cfg_prefixes 
            WHERE is_valid IS TRUE
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut prefix_map = HashMap::with_capacity(rows.len());
        
        for row in rows {
            let prefix: String = row.try_get("prefix")?;
            let country_id: Option<i32> = row.try_get("country_id")?;
            let operator_id: Option<i32> = row.try_get("operator_id")?;

            prefix_map.insert(prefix, (country_id, operator_id));
        }

        Ok(prefix_map)
    }

    pub fn lookup(&self, mut number: String) -> Prefixes {
        // Normalize the input by removing leading '+'
        if number.starts_with('+') {
            number = number.chars().skip(1).collect();
        }

        // Perform longest prefix match
        for i in (1..=number.len()).rev() {
            let prefix = &number[..i];
            if let Some(&(country_id, operator_id)) = self.prefix_map.get(prefix) {
                return Prefixes {
                    prefix: prefix.to_string(),
                    country_id,
                    operator_id,
                };
            }
        }

        // No match found
        Prefixes::default()
    }

    // Additional utility methods
    pub fn is_empty(&self) -> bool {
        self.prefix_map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.prefix_map.len()
    }

    pub fn contains_prefix(&self, prefix: &str) -> bool {
        self.prefix_map.contains_key(prefix)
    }

    // Fixed lifetime issue
    pub fn get_matching_prefixes<'a>(&self, number: &'a str) -> Vec<&'a str> {
        let normalized = number.trim_start_matches('+');
        let mut matches = Vec::new();

        for i in (1..=normalized.len()).rev() {
            let prefix = &normalized[..i];
            if self.prefix_map.contains_key(prefix) {
                matches.push(prefix);
            }
        }

        matches
    }
}

#[derive(Debug, Clone)]
pub struct LoadMetrics {
    pub prefix_count: usize,
    pub load_duration: std::time::Duration,
}

// Optional: Async refresh capability
impl PrefixLookup {
    pub async fn refresh(&mut self, pool: &Pool<Postgres>) -> Result<LoadMetrics, AppError> {
        use std::time::Instant;
        
        let start_time = Instant::now();
        let new_prefix_map = Self::load_prefixes_from_db(pool).await?;
        let load_duration = start_time.elapsed();

        self.prefix_map = new_prefix_map;

        Ok(LoadMetrics {
            prefix_count: self.prefix_map.len(),
            load_duration,
        })
    }
}
