pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GuardrailUnknownVerdict {}

impl GuardrailUnknownVerdict {
    pub fn builder() -> GuardrailUnknownVerdictBuilder {
        <GuardrailUnknownVerdictBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailUnknownVerdictBuilder {}

impl GuardrailUnknownVerdictBuilder {
    /// Consumes the builder and constructs a [`GuardrailUnknownVerdict`].
    pub fn build(self) -> Result<GuardrailUnknownVerdict, BuildError> {
        Ok(GuardrailUnknownVerdict {})
    }
}
