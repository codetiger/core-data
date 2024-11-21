use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Payload {
    /// Storage type: inline or file
    #[serde(rename = "type")]
    pub payload_type: PayloadType,
    
    /// Actual content when stored inline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    
    /// URL for external content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    /// Content format
    pub format: PayloadFormat,
    
    /// Character encoding
    pub encoding: Encoding,
    
    /// Size in bytes
    pub size: i64,
}

impl Payload {
    pub fn new_inline(content: Option<serde_json::Value>, format: PayloadFormat, encoding: Encoding) -> Self {
        Self {
            payload_type: PayloadType::Inline,
            content,
            url: None,
            format,
            encoding,
            size: 0,
        }
    }

    pub fn new_file(url: Option<String>, format: PayloadFormat, encoding: Encoding, size: i64) -> Self {
        Self {
            payload_type: PayloadType::File,
            content: None,
            url,
            format,
            encoding,
            size,
        }
    }
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
