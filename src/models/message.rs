use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::models::payload::*;
use crate::models::auditlog::*;
use sonyflake::Sonyflake;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    /// Unique identifier for the message
    #[serde(rename = "id")]
    pub id: u64,

    pub parent_id: Option<String>,

    /// Message payload containing content or reference
    pub payload: Payload,
    
    /// Tenant identifier
    pub tenant: String,
    
    /// Origin of the message
    pub origin: String,
    
    /// Message data
    pub data: serde_json::Value,
    
    /// Progress information
    pub progress: Progress,

    pub audit: Vec<AuditLog>,
}

impl Message {
    pub fn new(payload: Payload, tenant: String, origin: String, message_alias: Option<String>) -> Self {
        let sf = Sonyflake::new().unwrap();
        let id = sf.next_id().unwrap();

        let alias = message_alias.unwrap_or_else(|| "Message".to_string());
        let description = alias
            .chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>() + &alias[1..])
            .unwrap_or_else(|| "Message".to_string()) + " created";
        let reason = "Initial message creation for ".to_string() + &alias.to_lowercase();

        let mut audit = AuditLog::new();
        audit.description = description;
        audit.changes.push(ChangeLog {
            field: "payload".to_string(),
            old_value: None,
            new_value: None,
            reason,
        });

        Self {
            id,
            parent_id: None,
            payload,
            tenant,
            origin,
            data: serde_json::Value::Null,
            progress: Progress {
                status: MessageStatus::Recieved,
                workflow_id: "".to_string(),
                prev_task: "".to_string(),
                prev_status_code: "".to_string(),
                timestamp: OffsetDateTime::now_utc(),
            },
            audit: vec![audit],
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Progress {
    pub status: MessageStatus,

    /// Workflow definition identifier
    pub workflow_id: String,

    /// Last completed task identifier
    pub prev_task: String,
    
    /// Status code from last task
    pub prev_status_code: String,
    
    /// Timestamp of last completion
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageStatus {
    Recieved,
    Processing,
    Completed,
    Failed,
}