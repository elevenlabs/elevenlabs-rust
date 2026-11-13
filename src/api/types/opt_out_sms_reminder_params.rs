pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OptOutSmsReminderParams {}

impl OptOutSmsReminderParams {
    pub fn builder() -> OptOutSmsReminderParamsBuilder {
        <OptOutSmsReminderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptOutSmsReminderParamsBuilder {}

impl OptOutSmsReminderParamsBuilder {
    /// Consumes the builder and constructs a [`OptOutSmsReminderParams`].
    pub fn build(self) -> Result<OptOutSmsReminderParams, BuildError> {
        Ok(OptOutSmsReminderParams {})
    }
}
