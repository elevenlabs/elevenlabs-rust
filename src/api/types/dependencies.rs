pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Dependencies {
    Dependencies1ItemList(Vec<Dependencies1Item>),
    Dependencies2ItemList(Vec<Dependencies2Item>),
    DependentPhoneNumberIdentifierList(Vec<DependentPhoneNumberIdentifier>),
}

impl Dependencies {
    pub fn is_dependencies1_item_list(&self) -> bool {
        matches!(self, Self::Dependencies1ItemList(_))
    }

    pub fn as_dependencies1_item_list(&self) -> Option<&Vec<Dependencies1Item>> {
        match self {
            Self::Dependencies1ItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_dependencies1_item_list(self) -> Option<Vec<Dependencies1Item>> {
        match self {
            Self::Dependencies1ItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_dependencies2_item_list(&self) -> bool {
        matches!(self, Self::Dependencies2ItemList(_))
    }

    pub fn as_dependencies2_item_list(&self) -> Option<&Vec<Dependencies2Item>> {
        match self {
            Self::Dependencies2ItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_dependencies2_item_list(self) -> Option<Vec<Dependencies2Item>> {
        match self {
            Self::Dependencies2ItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_dependent_phone_number_identifier_list(&self) -> bool {
        matches!(self, Self::DependentPhoneNumberIdentifierList(_))
    }

    pub fn as_dependent_phone_number_identifier_list(
        &self,
    ) -> Option<&Vec<DependentPhoneNumberIdentifier>> {
        match self {
            Self::DependentPhoneNumberIdentifierList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_dependent_phone_number_identifier_list(
        self,
    ) -> Option<Vec<DependentPhoneNumberIdentifier>> {
        match self {
            Self::DependentPhoneNumberIdentifierList(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for Dependencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dependencies1ItemList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Dependencies2ItemList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DependentPhoneNumberIdentifierList(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

impl Default for Dependencies {
    fn default() -> Self {
        Dependencies::Dependencies1ItemList(Default::default())
    }
}
