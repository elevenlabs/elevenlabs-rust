pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "source")]
#[non_exhaustive]
pub enum McpToolConfigOverrideUpdateRequestModelInputOverrides {
    #[serde(rename = "constant")]
    Constant {
        #[serde(flatten)]
        data: ConstantSchemaOverride,
    },

    #[serde(rename = "dynamic_variable")]
    DynamicVariable {
        #[serde(flatten)]
        data: DynamicVariableSchemaOverride,
    },

    #[serde(rename = "llm")]
    Llm {
        #[serde(flatten)]
        data: LlmSchemaOverride,
    },

    #[serde(rename = "omit")]
    Omit {
        #[serde(flatten)]
        data: OmitSchemaOverride,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl McpToolConfigOverrideUpdateRequestModelInputOverrides {
    pub fn is_constant(&self) -> bool {
        matches!(self, Self::Constant { .. })
    }

    pub fn as_constant(&self) -> Option<&ConstantSchemaOverride> {
        match self {
            Self::Constant { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_constant(self) -> Option<ConstantSchemaOverride> {
        match self {
            Self::Constant { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_dynamic_variable(&self) -> bool {
        matches!(self, Self::DynamicVariable { .. })
    }

    pub fn as_dynamic_variable(&self) -> Option<&DynamicVariableSchemaOverride> {
        match self {
            Self::DynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_dynamic_variable(self) -> Option<DynamicVariableSchemaOverride> {
        match self {
            Self::DynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_llm(&self) -> bool {
        matches!(self, Self::Llm { .. })
    }

    pub fn as_llm(&self) -> Option<&LlmSchemaOverride> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_llm(self) -> Option<LlmSchemaOverride> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_omit(&self) -> bool {
        matches!(self, Self::Omit { .. })
    }

    pub fn as_omit(&self) -> Option<&OmitSchemaOverride> {
        match self {
            Self::Omit { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_omit(self) -> Option<OmitSchemaOverride> {
        match self {
            Self::Omit { data } => Some(data),
            _ => None,
        }
    }
}
