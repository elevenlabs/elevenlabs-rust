pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TemplateRunInput {
    String(String),
    Boolean(bool),
    Integer(i64),
    Double(f64),
    TemplateInputReference(TemplateInputReference),
    TemplateRunInputElementList(Vec<TemplateRunInputElement>),
}

impl TemplateRunInput {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_template_input_reference(&self) -> bool {
        matches!(self, Self::TemplateInputReference(_))
    }

    pub fn as_template_input_reference(&self) -> Option<&TemplateInputReference> {
        match self {
            Self::TemplateInputReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_template_input_reference(self) -> Option<TemplateInputReference> {
        match self {
            Self::TemplateInputReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_template_run_input_element_list(&self) -> bool {
        matches!(self, Self::TemplateRunInputElementList(_))
    }

    pub fn as_template_run_input_element_list(&self) -> Option<&Vec<TemplateRunInputElement>> {
        match self {
            Self::TemplateRunInputElementList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_template_run_input_element_list(self) -> Option<Vec<TemplateRunInputElement>> {
        match self {
            Self::TemplateRunInputElementList(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for TemplateRunInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Boolean(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Integer(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Double(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TemplateInputReference(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TemplateRunInputElementList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for TemplateRunInput {
    fn default() -> Self {
        TemplateRunInput::String(Default::default())
    }
}
