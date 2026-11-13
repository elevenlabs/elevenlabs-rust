pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidateUserVerificationCodeParams {}

impl ValidateUserVerificationCodeParams {
    pub fn builder() -> ValidateUserVerificationCodeParamsBuilder {
        <ValidateUserVerificationCodeParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateUserVerificationCodeParamsBuilder {}

impl ValidateUserVerificationCodeParamsBuilder {
    /// Consumes the builder and constructs a [`ValidateUserVerificationCodeParams`].
    pub fn build(self) -> Result<ValidateUserVerificationCodeParams, BuildError> {
        Ok(ValidateUserVerificationCodeParams {})
    }
}
