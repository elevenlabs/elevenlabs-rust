pub use crate::prelude::*;

/// The owner's spoken answer to changes the assistant read back to them.
///
/// Runs in the agent runtime rather than Reception: it needs the conversation's
/// chat to check the owner actually answered after hearing the changes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfirmChangeParams {}

impl ConfirmChangeParams {
    pub fn builder() -> ConfirmChangeParamsBuilder {
        <ConfirmChangeParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmChangeParamsBuilder {}

impl ConfirmChangeParamsBuilder {
    /// Consumes the builder and constructs a [`ConfirmChangeParams`].
    pub fn build(self) -> Result<ConfirmChangeParams, BuildError> {
        Ok(ConfirmChangeParams {})
    }
}
