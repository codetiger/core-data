use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
pub struct Payload {
    /// Storage type: inline or file
    #[serde(rename = "type")]
    #[builder(default = "PayloadType::Inline")]
    pub payload_type: PayloadType,
    
    /// Actual content when stored inline
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub content: Option<serde_json::Value>,
    
    /// URL for external content
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub url: Option<String>,
    
    /// Content format
    #[builder(default = "PayloadFormat::Xml")]
    pub format: PayloadFormat,
    
    /// Character encoding
    #[builder(default = "Encoding::Utf8")]
    pub encoding: Encoding,
    
    /// Size in bytes
    #[builder(default = "0")]
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PayloadType {
    Inline,
    File,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PayloadFormat {
    Xml,
    Json,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Encoding {
    #[serde(rename = "UTF-8")]
    Utf8,
    #[serde(rename = "UTF-16")]
    Utf16,
    #[serde(rename = "UTF-32")]
    Utf32,
    #[serde(rename = "ASCII")]
    Ascii,
    #[serde(rename = "ISO-8859-1")]
    Latin1,
}
