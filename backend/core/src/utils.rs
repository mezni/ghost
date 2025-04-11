use crate::db::DBManager;
use crate::entities::Prefixes;
use crate::errors::AppError;
use std::collections::HashMap;

pub async fn prefix_map(
    db_manager: &DBManager,
) -> Result<HashMap<String, (Option<i32>, Option<i32>)>, AppError> {
    let prefixes = db_manager.select_all_prefixes().await?;

    let prefix_map: HashMap<String, (Option<i32>, Option<i32>)> = prefixes
        .into_iter()
        .map(|p| (p.prefix, (p.country_id, p.operator_id)))
        .collect();

    Ok(prefix_map)
}

pub fn lookup(prefix_map: &HashMap<String, (Option<i32>, Option<i32>)>, mut s: String) -> Prefixes {
    while !s.is_empty() {
        if let Some((country_id, operator_id)) = prefix_map.get(&s) {
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
