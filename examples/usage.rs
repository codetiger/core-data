use core_data::models::message::*;

fn main() {
    // Create a Message instance using the builder pattern
    let message = MessageBuilder::default()
        .build()
        .unwrap();

    // Serialize the Message instance to a JSON string
    let serialized = serde_json::to_string_pretty(&message).unwrap();
    println!("Serialized Message:\n{}", serialized);
}