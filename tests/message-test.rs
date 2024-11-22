use serde_json::json;
use core_data::models::message::*;
use core_data::models::payload::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_message_creation_and_parse() {
        // Read test XML file
        let xml_bytes = fs::read("examples/pacs008_001_07_cct_outgoing.xml")
            .expect("Failed to read test XML file");

        // Create payload using constructor
        let payload = Payload::new_inline(
            Some(xml_bytes.clone()),
            PayloadFormat::Xml,
            PayloadSchema::ISO20022,
            Encoding::Utf8,
        );

        // Create message using constructor
        let mut message = Message::new(
            payload,
            "test_tenant".to_string(),
            "test_origin".to_string(),
            Some("TestMessage".to_string()),
        );

        // Verify initial state
        assert!(message.data.is_null());
        assert!(message.audit.len() == 1); // Should have creation audit
        assert!(message.audit[0].description.contains("TestMessage created"));

        // Parse the message
        message.parse(Some("Test XML parsing".to_string()))
            .expect("Failed to parse XML message");

        // Verify parsed state
        assert!(!message.data.is_null());
        assert!(message.audit.len() == 2); // Creation + parse audit
        assert_eq!(message.audit[1].description, "Test XML parsing");
    }

    #[test]
    fn test_message_enrichment() {
        // Create empty message
        let mut message = Message::new(
            Payload::new_inline(None, PayloadFormat::Json, PayloadSchema::ISO20022, Encoding::Utf8),
            "test_tenant".to_string(),
            "test_origin".to_string(),
            None,
        );

        // Set initial data
        message.data = json!({
            "field1": "value1",
            "nested": {
                "field2": "value2"
            }
        });

        // Configure enrichment
        let config = vec![
            EnrichmentConfig {
                field: "data.field1".to_string(),
                rule: json!({"var": ["new_value1"]}),
                description: Some("Update field1".to_string()),
            },
            EnrichmentConfig {
                field: "data.nested.field2".to_string(),
                rule: json!({"var": ["new_value2"]}),
                description: Some("Update nested field2".to_string()),
            },
        ];

        let enrich_data = json!({
            "new_value1": "updated1",
            "new_value2": "updated2"
        });

        // Apply enrichment
        message.enrich(config, enrich_data, Some("Test enrichment".to_string()))
            .expect("Failed to enrich message");

        // Verify enriched state
        assert_eq!(message.data["field1"], "updated1");
        assert_eq!(message.data["nested"]["field2"], "updated2");
        assert!(message.audit.len() > 1);
        
        // Verify audit trail
        let enrich_audit = message.audit.last().unwrap();
        assert_eq!(enrich_audit.changes.len(), 2);
        assert!(enrich_audit.changes.iter().any(|c| c.field == "data.field1"));
        assert!(enrich_audit.changes.iter().any(|c| c.field == "data.nested.field2"));
    }

    #[test]
    fn test_invalid_enrichment() {
        let mut message = Message::new(
            Payload::new_inline(None, PayloadFormat::Json, PayloadSchema::ISO20022, Encoding::Utf8),
            "test_tenant".to_string(),
            "test_origin".to_string(),
            None,
        );

        let invalid_config = vec![EnrichmentConfig {
            field: "invalid.path".to_string(),
            rule: json!({"var": ["non_existent"]}),
            description: Some("Invalid update".to_string()),
        }];

        let result = message.enrich(invalid_config, json!({}), Some("Invalid enrichment".to_string()));
        assert!(result.is_err());
    }
}