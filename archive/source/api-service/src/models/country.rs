use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub iso: String,
}
