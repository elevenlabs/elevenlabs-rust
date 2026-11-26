pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AgentWorkflowResponseModelNodes {
    #[serde(rename = "end")]
    End {
        #[serde(flatten)]
        data: WorkflowEndNodeModelOutput,
    },

    #[serde(rename = "override_agent")]
    OverrideAgent {
        #[serde(flatten)]
        data: WorkflowOverrideAgentNodeModelOutput,
    },

    #[serde(rename = "phone_number")]
    PhoneNumber {
        #[serde(flatten)]
        data: WorkflowPhoneNumberNodeModelOutput,
    },

    #[serde(rename = "say")]
    Say {
        #[serde(flatten)]
        data: WorkflowSayNodeModelOutput,
    },

    #[serde(rename = "standalone_agent")]
    StandaloneAgent {
        #[serde(flatten)]
        data: WorkflowStandaloneAgentNodeModelOutput,
    },

    #[serde(rename = "start")]
    Start {
        #[serde(flatten)]
        data: WorkflowStartNodeModelOutput,
    },

    #[serde(rename = "start_procedure")]
    StartProcedure {
        #[serde(flatten)]
        data: WorkflowStartProcedureNodeModelOutput,
    },

    #[serde(rename = "tool")]
    Tool {
        #[serde(flatten)]
        data: WorkflowToolNodeModelOutput,
    },

    #[serde(rename = "update_state")]
    UpdateState {
        #[serde(flatten)]
        data: WorkflowUpdateStateNodeModelOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AgentWorkflowResponseModelNodes {
    pub fn is_end(&self) -> bool {
        matches!(self, Self::End { .. })
    }

    pub fn as_end(&self) -> Option<&WorkflowEndNodeModelOutput> {
        match self {
            Self::End { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end(self) -> Option<WorkflowEndNodeModelOutput> {
        match self {
            Self::End { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_override_agent(&self) -> bool {
        matches!(self, Self::OverrideAgent { .. })
    }

    pub fn as_override_agent(&self) -> Option<&WorkflowOverrideAgentNodeModelOutput> {
        match self {
            Self::OverrideAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_override_agent(self) -> Option<WorkflowOverrideAgentNodeModelOutput> {
        match self {
            Self::OverrideAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_phone_number(&self) -> bool {
        matches!(self, Self::PhoneNumber { .. })
    }

    pub fn as_phone_number(&self) -> Option<&WorkflowPhoneNumberNodeModelOutput> {
        match self {
            Self::PhoneNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phone_number(self) -> Option<WorkflowPhoneNumberNodeModelOutput> {
        match self {
            Self::PhoneNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_say(&self) -> bool {
        matches!(self, Self::Say { .. })
    }

    pub fn as_say(&self) -> Option<&WorkflowSayNodeModelOutput> {
        match self {
            Self::Say { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_say(self) -> Option<WorkflowSayNodeModelOutput> {
        match self {
            Self::Say { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_standalone_agent(&self) -> bool {
        matches!(self, Self::StandaloneAgent { .. })
    }

    pub fn as_standalone_agent(&self) -> Option<&WorkflowStandaloneAgentNodeModelOutput> {
        match self {
            Self::StandaloneAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_standalone_agent(self) -> Option<WorkflowStandaloneAgentNodeModelOutput> {
        match self {
            Self::StandaloneAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_start(&self) -> bool {
        matches!(self, Self::Start { .. })
    }

    pub fn as_start(&self) -> Option<&WorkflowStartNodeModelOutput> {
        match self {
            Self::Start { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_start(self) -> Option<WorkflowStartNodeModelOutput> {
        match self {
            Self::Start { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_start_procedure(&self) -> bool {
        matches!(self, Self::StartProcedure { .. })
    }

    pub fn as_start_procedure(&self) -> Option<&WorkflowStartProcedureNodeModelOutput> {
        match self {
            Self::StartProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_start_procedure(self) -> Option<WorkflowStartProcedureNodeModelOutput> {
        match self {
            Self::StartProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_tool(&self) -> bool {
        matches!(self, Self::Tool { .. })
    }

    pub fn as_tool(&self) -> Option<&WorkflowToolNodeModelOutput> {
        match self {
            Self::Tool { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_tool(self) -> Option<WorkflowToolNodeModelOutput> {
        match self {
            Self::Tool { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_state(&self) -> bool {
        matches!(self, Self::UpdateState { .. })
    }

    pub fn as_update_state(&self) -> Option<&WorkflowUpdateStateNodeModelOutput> {
        match self {
            Self::UpdateState { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_state(self) -> Option<WorkflowUpdateStateNodeModelOutput> {
        match self {
            Self::UpdateState { data } => Some(data),
            _ => None,
        }
    }
}
