use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Technology {
    pub id: i32,
    pub technology: String,
    pub created_by: String,    
    pub created_at: NaiveDateTime,
    pub updated_by: String,    
    pub updated_at: NaiveDateTime,    
}

