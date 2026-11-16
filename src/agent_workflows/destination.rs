//! Where a `transfer()` node hands the conversation off to (kept in wire format).

use serde_json::{json, Value};

/// Where a `transfer()` node hands the conversation off to.
#[derive(Debug, Clone, PartialEq)]
pub struct Destination {
    model: Value,
}

impl Destination {
    fn new(model: Value) -> Destination {
        Destination { model }
    }

    /// A phone number in E.164 format.
    pub fn phone(phone_number: &str) -> Destination {
        Destination::new(json!({
            "type": "phone_number",
            "transfer_destination": { "type": "phone", "phone_number": phone_number }
        }))
    }

    /// A phone number read from a dynamic variable at transfer time.
    pub fn phone_variable(variable_name: &str) -> Destination {
        Destination::new(json!({
            "type": "phone_number",
            "transfer_destination": { "type": "phone_dynamic_variable", "phone_number": variable_name }
        }))
    }

    /// A SIP URI.
    pub fn sip(sip_uri: &str) -> Destination {
        Destination::new(json!({
            "type": "phone_number",
            "transfer_destination": { "type": "sip_uri", "sip_uri": sip_uri }
        }))
    }

    /// A SIP URI read from a dynamic variable at transfer time.
    pub fn sip_variable(variable_name: &str) -> Destination {
        Destination::new(json!({
            "type": "phone_number",
            "transfer_destination": { "type": "sip_uri_dynamic_variable", "sip_uri": variable_name }
        }))
    }

    /// Another ElevenLabs agent. `node_id` is the node in its workflow to start from.
    pub fn agent(agent_id: &str, node_id: Option<&str>) -> Destination {
        let mut model = json!({ "type": "standalone_agent", "agent_id": agent_id });
        if let Some(node_id) = node_id {
            model["node_id"] = json!(node_id);
        }
        Destination::new(model)
    }

    /// The node fields this destination sets, in API wire format.
    pub fn to_wire(&self) -> Value {
        self.model.clone()
    }
}
