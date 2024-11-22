use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use sonyflake::Sonyflake;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuditLog {
    /// Unique identifier for the audit log
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,

    pub workflow: String,

    pub task: String,

    pub description: String,

    pub hash: String,

    pub service: String,

    pub instance: String,

    pub version: String,

    pub changes: Vec<ChangeLog>,
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLog {
    pub fn new() -> Self {
        let sf = Sonyflake::new().unwrap();
        let id = sf.next_id().unwrap();
        let timestamp = OffsetDateTime::now_utc();
        AuditLog {
            id,
            timestamp,
            workflow: "".to_string(),
            task: "".to_string(),
            description: "".to_string(),
            hash: "".to_string(),
            service: "".to_string(),
            instance: "".to_string(),
            version: "".to_string(),
            changes: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChangeLog {
    pub field: String,

    pub old_value: Option<serde_json::Value>,

    pub new_value: Option<serde_json::Value>,

    pub reason: String,
}

impl Default for ChangeLog {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeLog {
    pub fn new() -> Self {
        ChangeLog {
            field: "".to_string(),
            old_value: None,
            new_value: None,
            reason: "".to_string(),
        }
    }
}