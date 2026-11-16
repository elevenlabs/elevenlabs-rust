pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateServiceQuoteRequestParams {}

impl CreateServiceQuoteRequestParams {
    pub fn builder() -> CreateServiceQuoteRequestParamsBuilder {
        <CreateServiceQuoteRequestParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServiceQuoteRequestParamsBuilder {}

impl CreateServiceQuoteRequestParamsBuilder {
    /// Consumes the builder and constructs a [`CreateServiceQuoteRequestParams`].
    pub fn build(self) -> Result<CreateServiceQuoteRequestParams, BuildError> {
        Ok(CreateServiceQuoteRequestParams {})
    }
}
