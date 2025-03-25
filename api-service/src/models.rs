use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::countries;

#[derive(Queryable, Serialize)]
pub struct Countries {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = countries)]
pub struct NewCountry {
    pub name: String,
}
