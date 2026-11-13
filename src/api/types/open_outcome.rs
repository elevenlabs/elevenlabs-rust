pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenOutcome {}

impl OpenOutcome {
    pub fn builder() -> OpenOutcomeBuilder {
        <OpenOutcomeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenOutcomeBuilder {}

impl OpenOutcomeBuilder {
    /// Consumes the builder and constructs a [`OpenOutcome`].
    pub fn build(self) -> Result<OpenOutcome, BuildError> {
        Ok(OpenOutcome {})
    }
}
