use serde::{Deserialize, Serialize};
use crate::models::task::*;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Workflow {
    pub name: String,

    pub description: String,

    pub version: u16,

    pub tags: Vec<String>,

    #[serde(rename = "status")]
    pub status: WorkflowStatus,

    pub tasks: Vec<Task>,

    pub condition: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WorkflowStatus {
    Draft,
    Active,
    Deprecated,
}


