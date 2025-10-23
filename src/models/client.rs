use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Client {
    pub client_id: Uuid,
    pub auth_user_id: Uuid,
    pub username: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}