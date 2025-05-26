// repositories.rs
use crate::models::{User, Session};
use crate::db::PgConn;

pub struct UserRepository {
    conn: PgConn,
}

impl UserRepository {
    pub fn new(conn: PgConn) -> Self {
        UserRepository { conn }
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = $1";
        let row = self.conn.query(query, &[&id]).await?;
        let user = User::from_row(row.get(0));
        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE username = $1";
        let row = self.conn.query(query, &[&username]).await?;
        let user = User::from_row(row.get(0));
        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> Result<(), String> {
        let query = "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)";
        self.conn.execute(query, &[&user.username, &user.email, &user.password_hash]).await?;
        Ok(())
    }

    pub async fn update_user(&self, user: &User) -> Result<(), String> {
        let query = "UPDATE users SET username = $1, email = $2, password_hash = $3 WHERE id = $4";
        self.conn.execute(query, &[&user.username, &user.email, &user.password_hash, &user.id]).await?;
        Ok(())
    }
}

pub struct SessionRepository {
    conn: PgConn,
}

impl SessionRepository {
    pub fn new(conn: PgConn) -> Self {
        SessionRepository { conn }
    }

    pub async fn get_session_by_token(&self, token: &str) -> Result<Session, String> {
        let query = "SELECT id, user_id, token, expires_at, created_at FROM sessions WHERE token = $1";
        let row = self.conn.query(query, &[&token]).await?;
        let session = Session::from_row(row.get(0));
        Ok(session)
    }

    pub async fn create_session(&self, session: &Session) -> Result<(), String> {
        let query = "INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, $3)";
        self.conn.execute(query, &[&session.user_id, &session.token, &session.expires_at]).await?;
        Ok(())
    }

    pub async fn delete_session(&self, token: &str) -> Result<(), String> {
        let query = "DELETE FROM sessions WHERE token = $1";
        self.conn.execute(query, &[&token]).await?;
        Ok(())
    }
}