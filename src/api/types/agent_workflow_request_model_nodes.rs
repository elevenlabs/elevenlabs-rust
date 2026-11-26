pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AgentWorkflowRequestModelNodes {
    #[serde(rename = "end")]
    End {
        #[serde(flatten)]
        data: WorkflowEndNodeModelInput,
    },

    #[serde(rename = "override_agent")]
    OverrideAgent {
        #[serde(flatten)]
        data: WorkflowOverrideAgentNodeModelInput,
    },

    #[serde(rename = "phone_number")]
    PhoneNumber {
        #[serde(flatten)]
        data: WorkflowPhoneNumberNodeModelInput,
    },

    #[serde(rename = "say")]
    Say {
        #[serde(flatten)]
        data: WorkflowSayNodeModelInput,
    },

    #[serde(rename = "standalone_agent")]
    StandaloneAgent {
        #[serde(flatten)]
        data: WorkflowStandaloneAgentNodeModelInput,
    },

    #[serde(rename = "start")]
    Start {
        #[serde(flatten)]
        data: WorkflowStartNodeModelInput,
    },

    #[serde(rename = "start_procedure")]
    StartProcedure {
        #[serde(flatten)]
        data: WorkflowStartProcedureNodeModelInput,
    },

    #[serde(rename = "tool")]
    Tool {
        #[serde(flatten)]
        data: WorkflowToolNodeModelInput,
    },

    #[serde(rename = "update_state")]
    UpdateState {
        #[serde(flatten)]
        data: WorkflowUpdateStateNodeModelInput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AgentWorkflowRequestModelNodes {
    pub fn is_end(&self) -> bool {
        matches!(self, Self::End { .. })
    }

    pub fn as_end(&self) -> Option<&WorkflowEndNodeModelInput> {
        match self {
            Self::End { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_end(self) -> Option<WorkflowEndNodeModelInput> {
        match self {
            Self::End { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_override_agent(&self) -> bool {
        matches!(self, Self::OverrideAgent { .. })
    }

    pub fn as_override_agent(&self) -> Option<&WorkflowOverrideAgentNodeModelInput> {
        match self {
            Self::OverrideAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_override_agent(self) -> Option<WorkflowOverrideAgentNodeModelInput> {
        match self {
            Self::OverrideAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_phone_number(&self) -> bool {
        matches!(self, Self::PhoneNumber { .. })
    }

    pub fn as_phone_number(&self) -> Option<&WorkflowPhoneNumberNodeModelInput> {
        match self {
            Self::PhoneNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_phone_number(self) -> Option<WorkflowPhoneNumberNodeModelInput> {
        match self {
            Self::PhoneNumber { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_say(&self) -> bool {
        matches!(self, Self::Say { .. })
    }

    pub fn as_say(&self) -> Option<&WorkflowSayNodeModelInput> {
        match self {
            Self::Say { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_say(self) -> Option<WorkflowSayNodeModelInput> {
        match self {
            Self::Say { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_standalone_agent(&self) -> bool {
        matches!(self, Self::StandaloneAgent { .. })
    }

    pub fn as_standalone_agent(&self) -> Option<&WorkflowStandaloneAgentNodeModelInput> {
        match self {
            Self::StandaloneAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_standalone_agent(self) -> Option<WorkflowStandaloneAgentNodeModelInput> {
        match self {
            Self::StandaloneAgent { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_start(&self) -> bool {
        matches!(self, Self::Start { .. })
    }

    pub fn as_start(&self) -> Option<&WorkflowStartNodeModelInput> {
        match self {
            Self::Start { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_start(self) -> Option<WorkflowStartNodeModelInput> {
        match self {
            Self::Start { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_start_procedure(&self) -> bool {
        matches!(self, Self::StartProcedure { .. })
    }

    pub fn as_start_procedure(&self) -> Option<&WorkflowStartProcedureNodeModelInput> {
        match self {
            Self::StartProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_start_procedure(self) -> Option<WorkflowStartProcedureNodeModelInput> {
        match self {
            Self::StartProcedure { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_tool(&self) -> bool {
        matches!(self, Self::Tool { .. })
    }

    pub fn as_tool(&self) -> Option<&WorkflowToolNodeModelInput> {
        match self {
            Self::Tool { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_tool(self) -> Option<WorkflowToolNodeModelInput> {
        match self {
            Self::Tool { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_update_state(&self) -> bool {
        matches!(self, Self::UpdateState { .. })
    }

    pub fn as_update_state(&self) -> Option<&WorkflowUpdateStateNodeModelInput> {
        match self {
            Self::UpdateState { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_update_state(self) -> Option<WorkflowUpdateStateNodeModelInput> {
        match self {
            Self::UpdateState { data } => Some(data),
            _ => None,
        }
    }
}
