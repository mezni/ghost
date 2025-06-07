use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub id: Option<i32>,
    pub name: String,
    pub code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
}

impl Country {
    pub fn builder(name: impl Into<String>, code: impl Into<String>) -> CountryBuilder {
        CountryBuilder::new(name.into(), code.into())
    }

    pub fn update(&mut self, name: Option<String>, code: Option<String>, updated_by: String) {
        if let Some(name) = name {
            self.name = capitalize_first_letter(&name);
        }

        if let Some(code) = code {
            self.code = code.trim().to_uppercase();
        }

        self.updated_by = Some(updated_by);
        self.updated_at = Some(Utc::now());
    }
}

pub struct CountryBuilder {
    id: Option<i32>,
    name: String,
    code: String,
    created_at: Option<DateTime<Utc>>,
    created_by: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<String>,
}

impl CountryBuilder {
    fn new(name: String, code: String) -> Self {
        let name = capitalize_first_letter(&name);
        let code = code.trim().to_uppercase();

        Self {
            id: None,
            name,
            code,
            created_at: Some(Utc::now()),
            created_by: None,
            updated_at: None,
            updated_by: None,
        }
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn created_by(mut self, creator: impl Into<String>) -> Self {
        self.created_by = Some(creator.into());
        self
    }

    pub fn updated_at(mut self, time: DateTime<Utc>) -> Self {
        self.updated_at = Some(time);
        self
    }

    pub fn updated_by(mut self, updater: impl Into<String>) -> Self {
        self.updated_by = Some(updater.into());
        self
    }

    pub fn build(self) -> Country {
        Country {
            id: self.id,
            name: self.name,
            code: self.code,
            created_at: self.created_at,
            created_by: self.created_by,
            updated_at: self.updated_at,
            updated_by: self.updated_by,
        }
    }
}

fn capitalize_first_letter(input: &str) -> String {
    let mut c = input.trim().chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
    }
}

#[async_trait]
pub trait CountryRepository: Send + Sync {
    async fn insert(&self, country: Country) -> Result<Country, String>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Country>, String>;
    async fn update(&self, country: Country) -> Result<(), String>;
    async fn delete(&self, id: i32) -> Result<(), String>;
    async fn list(&self) -> Result<Vec<Country>, String>;
}
