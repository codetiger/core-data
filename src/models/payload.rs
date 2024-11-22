use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Payload {
    /// Storage type: inline or file
    pub storage: StorageType,
    
    /// Actual content when stored inline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<u8>>,
    
    /// URL for external content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    /// Content format
    pub format: PayloadFormat,

    pub schema: PayloadSchema,
    
    /// Character encoding
    pub encoding: Encoding,
    
    /// Size in bytes
    pub size: i64,
}

impl Payload {
    pub fn new_inline(content: Option<Vec<u8>>, format: PayloadFormat, schema: PayloadSchema, encoding: Encoding) -> Self {
        Self {
            storage: StorageType::Inline,
            content,
            url: None,
            format,
            schema,
            encoding,
            size: 0,
        }
    }

    pub fn new_file(url: Option<String>, format: PayloadFormat, schema: PayloadSchema, encoding: Encoding, size: i64) -> Self {
        Self {
            storage: StorageType::File,
            content: None,
            url,
            format,
            schema,
            encoding,
            size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StorageType {
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
#[serde(rename_all = "lowercase")]
pub enum PayloadSchema {
    ISO20022,
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
