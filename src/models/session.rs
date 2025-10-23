use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuidl;
use secrecy::SecretString;

// Your proposed Session struct is perfect for storing an audit record of the session in your primary database (PostgreSQL/SQLx).
// This is not required for authentication, but it is necessary for security features like showing the user "Active Devices" 
// or immediately revoking a specific session.


#[derive(Debug, Deserialize, Serialize, )]
pub struct Session {
    pub session_id: Uuid,
    pub auth_user_id: Uuid,
    pub refresh_token_hash: SecretString,
    pub user_agent: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

