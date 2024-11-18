use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
pub struct Task {
    #[builder(default = "String::from(\"\")")]
    pub task_id: String,

    #[builder(default = "String::from(\"\")")]
    pub name: String,

    #[builder(default = "String::from(\"\")")]
    pub description: String,

    #[builder(default = "serde_json::json!({})")]
    pub trigger_condition: serde_json::Value,

    #[builder(default = "FunctionType::Validate")]
    pub function: FunctionType,

    #[builder(default = "serde_json::json!({})")]
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FunctionType {
    Validate,
    Enrich,
    Publish,
}
