use serde_json::json;
use time::OffsetDateTime;
use core_data::models::message::*;
use core_data::models::payload::*;
use sonyflake::Sonyflake;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_invalid_timestamp() {
        let progress = Progress {
            status: MessageStatus::Recieved,
            workflow_id: String::new(),
            prev_task: String::new(),
            prev_status_code: String::new(),
            timestamp: OffsetDateTime::UNIX_EPOCH - time::Duration::seconds(1),
        };

        let message = Message {
            id: Sonyflake::new().unwrap().next_id().unwrap(),
            parent_id: None,
            payload: Payload::new_inline(None, PayloadFormat::Json, Encoding::Utf8),
            tenant: String::new(),
            origin: String::new(),
            data: serde_json::Value::Null,
            progress,
            audit: vec![],
        };

        assert!(message.progress.timestamp < OffsetDateTime::UNIX_EPOCH);
    }

    #[test]
    fn test_message_large_payload() {
        let large_content = json!(vec!["a"; 10_000]);
        let content_size = serde_json::to_string(&large_content).unwrap().len() as i64;
        let payload = Payload {
            payload_type: PayloadType::Inline,
            content: Some(large_content.clone()),
            url: None,
            format: PayloadFormat::Json,
            encoding: Encoding::Utf8,
            size: content_size,
        };

        let message = Message {
            id: Sonyflake::new().unwrap().next_id().unwrap(),
            parent_id: None,
            payload,
            tenant: String::new(),
            origin: String::new(),
            data: serde_json::Value::Null,
            progress: Progress {
                status: MessageStatus::Recieved,
                workflow_id: String::new(),
                prev_task: String::new(),
                prev_status_code: String::new(),
                timestamp: OffsetDateTime::now_utc(),
            },
            audit: vec![],
        };

        assert_eq!(message.payload.content, Some(large_content));
        assert_eq!(message.payload.size, content_size);
    }
}