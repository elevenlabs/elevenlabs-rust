pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TestsCreateRequest {
    #[serde(rename = "llm")]
    Llm {
        #[serde(flatten)]
        data: CreateResponseUnitTestRequest,
    },

    #[serde(rename = "tool")]
    Tool {
        #[serde(flatten)]
        data: CreateToolCallUnitTestRequest,
    },

    #[serde(rename = "simulation")]
    Simulation {
        #[serde(flatten)]
        data: CreateSimulationTestRequest,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TestsCreateRequest {
    pub fn is_llm(&self) -> bool {
        matches!(self, Self::Llm { .. })
    }

    pub fn as_llm(&self) -> Option<&CreateResponseUnitTestRequest> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_llm(self) -> Option<CreateResponseUnitTestRequest> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_tool(&self) -> bool {
        matches!(self, Self::Tool { .. })
    }

    pub fn as_tool(&self) -> Option<&CreateToolCallUnitTestRequest> {
        match self {
            Self::Tool { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_tool(self) -> Option<CreateToolCallUnitTestRequest> {
        match self {
            Self::Tool { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_simulation(&self) -> bool {
        matches!(self, Self::Simulation { .. })
    }

    pub fn as_simulation(&self) -> Option<&CreateSimulationTestRequest> {
        match self {
            Self::Simulation { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_simulation(self) -> Option<CreateSimulationTestRequest> {
        match self {
            Self::Simulation { data } => Some(data),
            _ => None,
        }
    }
}
