pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAssetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl ListAssetsParams {
    pub fn builder() -> ListAssetsParamsBuilder {
        <ListAssetsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAssetsParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl ListAssetsParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAssetsParams`].
    pub fn build(self) -> Result<ListAssetsParams, BuildError> {
        Ok(ListAssetsParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
