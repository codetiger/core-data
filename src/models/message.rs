use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use derive_builder::Builder;
use crate::models::payload::*;
use crate::models::auditlog::*;
use sonyflake::Sonyflake;

#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
pub struct Message {
    /// Unique identifier for the message
    #[serde(rename = "id")]
    #[builder(default = "Sonyflake::new().unwrap().next_id().unwrap()")]
    pub id: u64,

    #[builder(default = "None")]
    pub parent_id: Option<String>,

    /// Message payload containing content or reference
    #[builder(default = "PayloadBuilder::default().build().unwrap()")]
    pub payload: Payload,
    
    /// Tenant identifier
    #[builder(default = "String::from(\"\")")]
    pub tenant: String,
    
    /// Origin of the message
    #[builder(default = "String::from(\"\")")]
    pub origin: String,
    
    /// Message data
    #[builder(default = "serde_json::json!({})")]
    pub data: serde_json::Value,
    
    /// Progress information
    #[builder(default = "ProgressBuilder::default().build().unwrap()")]
    pub progress: Progress,

    #[builder(default = "Vec::new()")]
    pub audit: Vec<AuditLog>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
pub struct Progress {
    #[builder(default = "MessageStatus::Recieved")]
    pub status: MessageStatus,

    /// Workflow definition identifier
    #[builder(default = "String::from(\"\")")]
    pub workflow_id: String,

    /// Last completed task identifier
    #[builder(default = "String::from(\"\")")]
    pub prev_task: String,
    
    /// Status code from last task
    #[builder(default = "String::from(\"\")")]
    pub prev_status_code: String,
    
    /// Timestamp of last completion
    #[serde(with = "time::serde::iso8601")]
    #[builder(default = "OffsetDateTime::UNIX_EPOCH")]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageStatus {
    Recieved,
    Processing,
    Completed,
    Failed,
}