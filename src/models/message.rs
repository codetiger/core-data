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

    pub workflow_id: String,

    pub prev_task: String,
    
    pub prev_status_code: String,
    
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
    #[serde(rename = "id")]
    id: u64,

    parent_id: Option<String>,

    payload: Payload,
    
    tenant: String,
    
    origin: String,
    
    data: Value,

    metadata: Value,
    
    progress: Progress,

    audit: Vec<AuditLog>,
}

impl Message {
    pub fn audit(&self) -> &Vec<AuditLog> {
        &self.audit
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

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

        let change_log = ChangeLog::new("payload".to_string(), reason, None, None);
        let audit = AuditLog::new(
            String::new().to_string(), 
            String::new().to_string(), 
            description.to_string(),
            vec![change_log]
        );

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
        let mut changes = Vec::new();

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
                        let change = ChangeLog::new(
                            cfg.field.to_string(),
                            cfg.description.clone().unwrap_or_else(|| format!("Enriched field {}", cfg.field)), 
                            Some(old_value),
                            Some(value.clone())
                        );
                        changes.push(change);
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
    
        let audit_log = AuditLog::new(
            String::new().to_string(),
            String::new().to_string(),
            description.unwrap_or_else(|| "Enrichment applied".to_string()),
            changes
        );
        self.audit.push(audit_log);

        Ok(())
    }

    pub fn parse(&mut self, description: Option<String>) -> Result<(), FunctionResponseError> {
        const BUFFER_SIZE: usize = 32 * 1024; // 32KB buffer

        let buf_reader: Box<dyn BufRead> = if let Some(content) = self.payload.content() {
            Box::new(BufReader::with_capacity(
                BUFFER_SIZE,
                content
            ))
        } else if let Some(ref url) = self.payload.url() {
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

        match from_reader::<_, ISO20022Message>(buf_reader) {
            Ok(message) => {
                match message.validate() {
                    Ok(()) => {
                        self.data = serde_json::to_value(message).unwrap();
                        let change_log = ChangeLog::new(
                            "data".to_string(),
                            "ISO20022 message parsed".to_string(),
                            None,
                            None
                        );
                        let audit_log = AuditLog::new(
                            String::new().to_string(),
                            String::new().to_string(),
                            description.unwrap_or_else(|| "ISO20022 message parsed".to_string()),
                            vec![change_log]
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
