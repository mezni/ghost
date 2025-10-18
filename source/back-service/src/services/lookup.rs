use crate::core::errors::AppError;
use sqlx::{Pool, Postgres, Row};
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
        let records = sqlx::query(
            "SELECT prefix, country_id, operator_id FROM cfg_prefixes WHERE is_valid IS TRUE",
        )
        .fetch_all(pool)
        .await?;

        let mut prefix_map = HashMap::with_capacity(records.len());

        for record in records {
            let prefix: String = record.get("prefix");
            let country_id: Option<i32> = record.get("country_id");
            let operator_id: Option<i32> = record.get("operator_id");

            prefix_map.insert(prefix, (country_id, operator_id));
        }

        Ok(prefix_map)
    }

    pub fn lookup(&self, mut number: String) -> Prefixes {
        if number.starts_with('+') {
            number = number.chars().skip(1).collect();
        }

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

        Prefixes::default()
    }

    pub fn is_empty(&self) -> bool {
        self.prefix_map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.prefix_map.len()
    }

    pub fn contains_prefix(&self, prefix: &str) -> bool {
        self.prefix_map.contains_key(prefix)
    }

    pub fn get_matching_prefixes<'a>(&self, number: &'a str) -> Vec<&'a str> {
        let normalized = number.trim_start_matches('+');
        (1..=normalized.len())
            .rev()
            .filter_map(|i| {
                let prefix = &normalized[..i];
                if self.prefix_map.contains_key(prefix) {
                    Some(prefix)
                } else {
                    None
                }
            })
            .collect()
    }

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

#[derive(Debug, Clone)]
pub struct LoadMetrics {
    pub prefix_count: usize,
    pub load_duration: std::time::Duration,
}
