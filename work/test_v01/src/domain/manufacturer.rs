use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manufacturer {
    pub id: String,
    pub name: String,
    pub contact_info: Option<String>,
}

impl Manufacturer {
    pub fn new(name: String, contact_info: Option<String>) -> Self {
        let random_uuid = Uuid::new_v4();
        Self {
            id: random_uuid.to_string(),
            name,
            contact_info,
        }
    }
}

pub trait ManufacturerRepository {
    fn create(&self, manufacturer: &Manufacturer) -> Result<(), Box<dyn Error>>;
    fn get_all(&self) -> Result<Vec<Manufacturer>, Box<dyn Error>>;
    fn get_by_id(&self, id: &str) -> Result<Option<Manufacturer>, Box<dyn Error>>;
    fn update(&self, manufacturer: &Manufacturer) -> Result<(), Box<dyn Error>>;
    fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
