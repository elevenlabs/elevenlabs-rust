pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Threshold {
    Double(f64),
    Eagerness(Eagerness),
}

impl Threshold {
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

    pub fn is_eagerness(&self) -> bool {
        matches!(self, Self::Eagerness(_))
    }

    pub fn as_eagerness(&self) -> Option<&Eagerness> {
        match self {
            Self::Eagerness(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_eagerness(self) -> Option<Eagerness> {
        match self {
            Self::Eagerness(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for Threshold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Eagerness(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Threshold::Double(Default::default())
    }
}
