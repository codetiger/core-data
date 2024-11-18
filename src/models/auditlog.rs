use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use derive_builder::Builder;
use sonyflake::Sonyflake;

#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
pub struct AuditLog {
    /// Unique identifier for the audit log
    #[serde(rename = "id")]
    #[builder(default = "Sonyflake::new().unwrap().next_id().unwrap()")]
    pub id: u64,

    #[serde(with = "time::serde::iso8601")]
    #[builder(default = "OffsetDateTime::UNIX_EPOCH")]
    pub timestamp: OffsetDateTime,

    #[builder(default = "String::from(\"\")")]
    pub workflow: String,

    #[builder(default = "String::from(\"\")")]
    pub task: String,

    #[builder(default = "String::from(\"\")")]
    pub description: String,

    #[builder(default = "String::from(\"\")")]
    pub hash: String,

    #[builder(default = "String::from(\"\")")]
    pub service: String,

    #[builder(default = "String::from(\"\")")]
    pub instance: String,

    #[builder(default = "String::from(\"\")")]
    pub version: String,

    #[builder(default = "Vec::new()")]
    pub changes: Vec<ChangeLog>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
pub struct ChangeLog {
    #[builder(default = "String::from(\"\")")]
    pub field: String,

    #[builder(default = "None")]
    pub old_value: Option<serde_json::Value>,

    #[builder(default = "None")]
    pub new_value: Option<serde_json::Value>,

    #[builder(default = "String::from(\"\")")]
    pub reason: String,

}