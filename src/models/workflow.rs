use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use crate::models::task::*;


#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
pub struct Workflow {
    #[builder(default = "String::from(\"\")")]
    pub name: String,

    #[builder(default = "String::from(\"\")")]
    pub description: String,

    #[builder(default = "0")]
    pub version: u16,

    #[builder(default = "Vec::new()")]
    pub tags: Vec<String>,

    #[serde(rename = "status")]
    #[builder(default = "WorkflowStatus::Draft")]
    pub status: WorkflowStatus,

    #[builder(default = "Vec::new()")]
    pub tasks: Vec<Task>,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WorkflowStatus {
    Draft,
    Active,
    Deprecated,
}


