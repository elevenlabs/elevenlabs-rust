pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum LiteralJsonSchemaPropertyType {
    AnalysisPropertyType(AnalysisPropertyType),
    StringList(Vec<String>),
}

impl LiteralJsonSchemaPropertyType {
    pub fn is_analysis_property_type(&self) -> bool {
        matches!(self, Self::AnalysisPropertyType(_))
    }

    pub fn as_analysis_property_type(&self) -> Option<&AnalysisPropertyType> {
        match self {
            Self::AnalysisPropertyType(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_analysis_property_type(self) -> Option<AnalysisPropertyType> {
        match self {
            Self::AnalysisPropertyType(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for LiteralJsonSchemaPropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AnalysisPropertyType(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::StringList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for LiteralJsonSchemaPropertyType {
    fn default() -> Self {
        LiteralJsonSchemaPropertyType::StringList(Default::default())
    }
}
