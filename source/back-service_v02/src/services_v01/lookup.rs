use crate::core::errors::AppError;
use deadpool_postgres::Pool;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Prefixes {
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

pub struct PrefixLookup {
    prefix_map: HashMap<String, (Option<i32>, Option<i32>)>,
}

impl PrefixLookup {
    pub async fn new(pool: &Pool) -> Result<Self, AppError> {
        let prefix_map = Self::get_prefixes(pool).await?;
        Ok(PrefixLookup { prefix_map })
    }

    async fn get_prefixes(
        pool: &Pool,
    ) -> Result<HashMap<String, (Option<i32>, Option<i32>)>, AppError> {
        let client = pool.get().await?;
        let rows = client
            .query(
                "SELECT prefix, country_id, operator_id FROM cfg_prefixes WHERE is_valid IS TRUE",
                &[],
            )
            .await?;

        let mut prefixes = HashMap::new();
        for row in rows {
            let prefix: String = row.try_get("prefix").map_err(AppError::from)?;
            let country_id: Option<i32> = row.try_get("country_id").map_err(AppError::from)?;
            let operator_id: Option<i32> = row.try_get("operator_id").map_err(AppError::from)?;

            prefixes.insert(prefix, (country_id, operator_id));
        }

        Ok(prefixes)
    }

    pub fn lookup(&self, mut s: String) -> Prefixes {
        // Remove the leading "+" if it exists
        if s.starts_with('+') {
            s = s.trim_start_matches('+').to_string();
        }

        while !s.is_empty() {
            if let Some((country_id, operator_id)) = self.prefix_map.get(&s) {
                return Prefixes {
                    prefix: s.clone(),
                    country_id: *country_id,
                    operator_id: *operator_id,
                };
            }
            s.pop();
        }

        Prefixes {
            prefix: "".to_string(),
            country_id: None,
            operator_id: None,
        }
    }
}
