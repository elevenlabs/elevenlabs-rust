pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum McpToolConfigOverrideInputToolCallSound {
    ToolCallSoundType(ToolCallSoundType),
    ToolCallSound2_1(ToolCallSound2_1),
}

impl McpToolConfigOverrideInputToolCallSound {
    pub fn is_tool_call_sound_type(&self) -> bool {
        matches!(self, Self::ToolCallSoundType(_))
    }

    pub fn as_tool_call_sound_type(&self) -> Option<&ToolCallSoundType> {
        match self {
            Self::ToolCallSoundType(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tool_call_sound_type(self) -> Option<ToolCallSoundType> {
        match self {
            Self::ToolCallSoundType(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_tool_call_sound2_1(&self) -> bool {
        matches!(self, Self::ToolCallSound2_1(_))
    }

    pub fn as_tool_call_sound2_1(&self) -> Option<&ToolCallSound2_1> {
        match self {
            Self::ToolCallSound2_1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tool_call_sound2_1(self) -> Option<ToolCallSound2_1> {
        match self {
            Self::ToolCallSound2_1(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for McpToolConfigOverrideInputToolCallSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolCallSoundType(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ToolCallSound2_1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
