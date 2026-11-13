pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AstllmNodeInput {
    AstllmNodeInput1(AstllmNodeInput1),
    AstllmNodeInput2(AstllmNodeInput2),
}

impl<'de> serde::Deserialize<'de> for AstllmNodeInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

        let mut matched_literal = false;

        if value.get("type").and_then(serde_json::Value::as_str) == Some("llm") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<AstllmNodeInput1>(value.clone()) {
                return Ok(Self::AstllmNodeInput1(parsed));
            }
        }

        if value.get("type").and_then(serde_json::Value::as_str) == Some("llm") {
            matched_literal = true;
            if let Ok(parsed) = serde_json::from_value::<AstllmNodeInput2>(value.clone()) {
                return Ok(Self::AstllmNodeInput2(parsed));
            }
        }

        if !matched_literal {
            if let Ok(parsed) = serde_json::from_value::<AstllmNodeInput1>(value.clone()) {
                return Ok(Self::AstllmNodeInput1(parsed));
            }

            if let Ok(parsed) = serde_json::from_value::<AstllmNodeInput2>(value.clone()) {
                return Ok(Self::AstllmNodeInput2(parsed));
            }
        }

        Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum AstllmNodeInput",
        ))
    }
}

impl AstllmNodeInput {
    pub fn is_astllm_node_input1(&self) -> bool {
        matches!(self, Self::AstllmNodeInput1(_))
    }

    pub fn as_astllm_node_input1(&self) -> Option<&AstllmNodeInput1> {
        match self {
            Self::AstllmNodeInput1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_astllm_node_input1(self) -> Option<AstllmNodeInput1> {
        match self {
            Self::AstllmNodeInput1(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_astllm_node_input2(&self) -> bool {
        matches!(self, Self::AstllmNodeInput2(_))
    }

    pub fn as_astllm_node_input2(&self) -> Option<&AstllmNodeInput2> {
        match self {
            Self::AstllmNodeInput2(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_astllm_node_input2(self) -> Option<AstllmNodeInput2> {
        match self {
            Self::AstllmNodeInput2(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AstllmNodeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AstllmNodeInput1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AstllmNodeInput2(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
