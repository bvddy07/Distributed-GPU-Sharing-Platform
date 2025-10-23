use serde::{Deserialize};
use uuid::Uuid;
use serde_json::value,

#[derive(Debug, Clone, Deserialize)]
pub struct Host {
    pub host_id: Uuid,
    pub auth_user_id: Uuid,
    pub username: String,
    pub is_active: bool,
    pub location: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}