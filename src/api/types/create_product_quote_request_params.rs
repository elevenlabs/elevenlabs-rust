pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductQuoteRequestParams {}

impl CreateProductQuoteRequestParams {
    pub fn builder() -> CreateProductQuoteRequestParamsBuilder {
        <CreateProductQuoteRequestParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductQuoteRequestParamsBuilder {}

impl CreateProductQuoteRequestParamsBuilder {
    /// Consumes the builder and constructs a [`CreateProductQuoteRequestParams`].
    pub fn build(self) -> Result<CreateProductQuoteRequestParams, BuildError> {
        Ok(CreateProductQuoteRequestParams {})
    }
}
