use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::models::User;
use uuid::Uuid;
use argon2::{self, Config};

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new().connect(&db_url).await
}

pub async fn find_user(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}
