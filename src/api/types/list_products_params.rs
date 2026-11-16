pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListProductsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl ListProductsParams {
    pub fn builder() -> ListProductsParamsBuilder {
        <ListProductsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProductsParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl ListProductsParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProductsParams`].
    pub fn build(self) -> Result<ListProductsParams, BuildError> {
        Ok(ListProductsParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
