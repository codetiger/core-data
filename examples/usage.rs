use core_data::models::message::*;
use core_data::models::payload::*;

fn main() {
    // Create a Message instance using the builder pattern
    let message = Message::new(
        Payload::new_inline(
            Some(serde_json::json!({"key": "value"})),
            PayloadFormat::Json,
            Encoding::Utf8,
        ),
        "tenant".to_string(),
        "origin".to_string(),
        None,
    );

    // Serialize the Message instance to a JSON string
    let serialized = serde_json::to_string_pretty(&message).unwrap();
    println!("Serialized Message:\n{}", serialized);
}