use std::fs::File;
use std::io::{BufReader, BufRead};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::OffsetDateTime;
use sonyflake::Sonyflake;
use quick_xml::de::from_reader;

use datalogic_rs::JsonLogic;

use crate::models::payload::*;
use crate::models::auditlog::*;
use crate::models::errors::FunctionResponseError;
use crate::models::iso20022::ISO20022Message;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EnrichmentConfig {
    pub field: String,

    pub rule: Value,

    pub description: Option<String>,
}

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
    pub data: Value,

    pub metadata: Value,
    
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
            data: Value::Null,
            metadata: Value::Null,
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
    
    pub fn enrich(&mut self, config: Vec<EnrichmentConfig>, data: serde_json::Value, description: Option<String>) -> Result<(), FunctionResponseError> {
        let logic = JsonLogic::new();
        let mut audit_log = AuditLog::new();
        audit_log.description = description.unwrap_or_else(|| "Enrichment applied".to_string());
    
        for cfg in config {
            let value = logic.apply(&cfg.rule, &data).unwrap();
            let field_path: Vec<&str> = cfg.field.split('.').collect();
            let root_field = field_path[0];
    
            if root_field == "data" {
                let mut current = &mut self.data;
                for (i, part) in field_path.iter().enumerate().skip(1) {
                    if i == field_path.len() - 1 {
                        let old_value = current[part].take();
                        current[part] = value.clone();
                        audit_log.changes.push(ChangeLog {
                            field: cfg.field.clone(),
                            old_value: Some(old_value),
                            new_value: Some(value.clone()),
                            reason: cfg.description.clone().unwrap_or_else(|| format!("Enriched field {}", cfg.field)),
                        });
                    } else {
                        if !current[part].is_object() {
                            current[part] = json!({});
                        }
                        current = current.get_mut(part).unwrap();
                    }
                }
            } else {
                return Err(FunctionResponseError::new("Enrichment".to_string(), 400, "Invalid field path".to_string()));
            }
        }
    
        self.audit.push(audit_log);

        Ok(())
    }

    pub fn parse(&mut self, description: Option<String>) -> Result<(), FunctionResponseError> {
        const BUFFER_SIZE: usize = 32 * 1024; // 32KB buffer

        // Create a BufReader explicitly
        let buf_reader: Box<dyn BufRead> = if let Some(ref content) = self.payload.content {
            Box::new(BufReader::with_capacity(
                BUFFER_SIZE,
                content.as_slice()
            ))
        } else if let Some(ref url) = self.payload.url {
            let file = File::open(url).map_err(|e| {
                FunctionResponseError::new(
                    "Parse".to_string(),
                    400,
                    format!("File open error: {:?}", e)
                )
            })?;
            Box::new(BufReader::with_capacity(BUFFER_SIZE, file))
        } else {
            return Err(FunctionResponseError::new(
                "Parse".to_string(),
                400,
                "No content or URL provided".to_string()
            ));
        };

        // Parse using quick-xml with BufReader
        match from_reader::<_, ISO20022Message>(buf_reader) {
            Ok(message) => {
                match message.validate() {
                    Ok(()) => {
                        self.data = serde_json::to_value(message).unwrap();
                        let mut audit_log = AuditLog::new();
                        audit_log.description = description.unwrap_or_else(|| 
                            "ISO20022 message parsed".to_string()
                        );
                        self.audit.push(audit_log);
                        Ok(())
                    }
                    Err(validation_error) => {
                        Err(FunctionResponseError::new(
                            "Parse".to_string(),
                            400,
                            format!("Schema validation error: {:?}", validation_error)
                        ))
                    }
                }
            }
            Err(e) => Err(FunctionResponseError::new(
                "Parse".to_string(),
                400,
                format!("ISO20022 parsing error: {:?}", e)
            )),
        }
    }
}
