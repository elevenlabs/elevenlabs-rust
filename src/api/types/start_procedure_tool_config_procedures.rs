pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StartProcedureToolConfigProcedures {
    ProcedureVersionRef(ProcedureVersionRef),
    ProcedureDraftRef(ProcedureDraftRef),
}

impl StartProcedureToolConfigProcedures {
    pub fn is_procedure_version_ref(&self) -> bool {
        matches!(self, Self::ProcedureVersionRef(_))
    }

    pub fn as_procedure_version_ref(&self) -> Option<&ProcedureVersionRef> {
        match self {
            Self::ProcedureVersionRef(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_procedure_version_ref(self) -> Option<ProcedureVersionRef> {
        match self {
            Self::ProcedureVersionRef(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_procedure_draft_ref(&self) -> bool {
        matches!(self, Self::ProcedureDraftRef(_))
    }

    pub fn as_procedure_draft_ref(&self) -> Option<&ProcedureDraftRef> {
        match self {
            Self::ProcedureDraftRef(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_procedure_draft_ref(self) -> Option<ProcedureDraftRef> {
        match self {
            Self::ProcedureDraftRef(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for StartProcedureToolConfigProcedures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcedureVersionRef(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ProcedureDraftRef(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
