pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OptInSmsReminderParams {}

impl OptInSmsReminderParams {
    pub fn builder() -> OptInSmsReminderParamsBuilder {
        <OptInSmsReminderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptInSmsReminderParamsBuilder {}

impl OptInSmsReminderParamsBuilder {
    /// Consumes the builder and constructs a [`OptInSmsReminderParams`].
    pub fn build(self) -> Result<OptInSmsReminderParams, BuildError> {
        Ok(OptInSmsReminderParams {})
    }
}
