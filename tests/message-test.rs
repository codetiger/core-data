use time::OffsetDateTime;
use core_data::models::payload::*;
use core_data::models::auditlog::*;
use core_data::models::message::*;
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_builder_defaults() {
        let message = MessageBuilder::default().build().unwrap();
        
        assert!(message.id > 0);
        assert_eq!(message.parent_id, None);
        assert_eq!(message.payload.payload_type, PayloadType::Inline);
        assert_eq!(message.payload.content, None);
        assert_eq!(message.payload.url, None);
        assert_eq!(message.payload.format, PayloadFormat::Xml);
        assert_eq!(message.payload.encoding, Encoding::Utf8);
        assert_eq!(message.payload.size, 0);
        assert_eq!(message.tenant, "");
        assert_eq!(message.origin, "");
        assert_eq!(message.data, json!({}));
        assert_eq!(message.progress.status, MessageStatus::Recieved);
        assert_eq!(message.progress.workflow_id, "");
        assert_eq!(message.progress.prev_task, "");
        assert_eq!(message.progress.prev_status_code, "");
        assert_eq!(message.progress.timestamp, OffsetDateTime::UNIX_EPOCH);
        assert!(message.audit.is_empty());
    }

    #[test]
    fn test_message_custom_values() {
        let payload = PayloadBuilder::default()
            .payload_type(PayloadType::File)
            .url(Some(String::from("http://example.com")))
            .format(PayloadFormat::Json)
            .encoding(Encoding::Utf16)
            .size(1024)
            .build()
            .unwrap();

        let progress = ProgressBuilder::default()
            .status(MessageStatus::Processing)
            .workflow_id(String::from("workflow_123"))
            .prev_task(String::from("task_1"))
            .prev_status_code(String::from("200"))
            .timestamp(OffsetDateTime::now_utc())
            .build()
            .unwrap();

        let audit_log = AuditLogBuilder::default()
            .workflow(String::from("workflow_123"))
            .task(String::from("task_1"))
            .description(String::from("Task completed"))
            .hash(String::from("hash_123"))
            .service(String::from("service_1"))
            .instance(String::from("instance_1"))
            .version(String::from("v1"))
            .build()
            .unwrap();

        let message = MessageBuilder::default()
            .parent_id(Some(String::from("parent_123")))
            .payload(payload)
            .tenant(String::from("tenant_1"))
            .origin(String::from("origin_1"))
            .data(json!({"key": "value"}))
            .progress(progress)
            .audit(vec![audit_log])
            .build()
            .unwrap();

        assert_eq!(message.parent_id, Some(String::from("parent_123")));
        assert_eq!(message.payload.payload_type, PayloadType::File);
        assert_eq!(message.payload.url, Some(String::from("http://example.com")));
        assert_eq!(message.payload.format, PayloadFormat::Json);
        assert_eq!(message.payload.encoding, Encoding::Utf16);
        assert_eq!(message.payload.size, 1024);
        assert_eq!(message.tenant, String::from("tenant_1"));
        assert_eq!(message.origin, String::from("origin_1"));
        assert_eq!(message.data, json!({"key": "value"}));
        assert_eq!(message.progress.status, MessageStatus::Processing);
        assert_eq!(message.progress.workflow_id, String::from("workflow_123"));
        assert_eq!(message.progress.prev_task, String::from("task_1"));
        assert_eq!(message.progress.prev_status_code, String::from("200"));
        assert!(message.progress.timestamp > OffsetDateTime::UNIX_EPOCH);
        assert_eq!(message.audit.len(), 1);
        assert_eq!(message.audit[0].workflow, String::from("workflow_123"));
    }

    #[test]
    fn test_message_empty_payload() {
        let payload = PayloadBuilder::default()
            .payload_type(PayloadType::Inline)
            .content(None)
            .url(None)
            .format(PayloadFormat::Xml)
            .encoding(Encoding::Utf8)
            .size(0)
            .build()
            .unwrap();

        let message = MessageBuilder::default()
            .payload(payload)
            .build()
            .unwrap();

        assert_eq!(message.payload.payload_type, PayloadType::Inline);
        assert_eq!(message.payload.content, None);
        assert_eq!(message.payload.url, None);
        assert_eq!(message.payload.format, PayloadFormat::Xml);
        assert_eq!(message.payload.encoding, Encoding::Utf8);
        assert_eq!(message.payload.size, 0);
    }

    #[test]
    fn test_message_invalid_timestamp() {
        let progress = ProgressBuilder::default()
            .timestamp(OffsetDateTime::UNIX_EPOCH - time::Duration::seconds(1))
            .build()
            .unwrap();

        let message = MessageBuilder::default()
            .progress(progress)
            .build()
            .unwrap();

        assert!(message.progress.timestamp < OffsetDateTime::UNIX_EPOCH);
    }

    #[test]
    fn test_message_large_payload() {
        let large_content = serde_json::json!(vec!["a"; 10_000]);
        let content_size = serde_json::to_string(&large_content).unwrap().len() as i64;
        let payload = PayloadBuilder::default()
            .payload_type(PayloadType::Inline)
            .content(Some(large_content.clone()))
            .size(large_content.to_string().len() as i64)
            .build()
            .unwrap();

        let message = MessageBuilder::default()
            .payload(payload)
            .build()
            .unwrap();

        assert_eq!(message.payload.content, Some(large_content));
        assert_eq!(message.payload.size, content_size);
    }
}