use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use secrecy::SecretString;

#[derive(Debug, FromRow)]
pub struct AuthUser {
    pub auth_user_id: Uuid,
    pub auth_email: String,
    pub auth_password_hash: SecretString,
    pub username: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}