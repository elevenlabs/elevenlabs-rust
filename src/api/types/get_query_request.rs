pub use crate::prelude::*;

/// Query parameters for Retrieve Voice Sample Audio
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetQueryRequest {
    /// If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
}

impl GetQueryRequest {
    pub fn builder() -> GetQueryRequestBuilder {
        <GetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetQueryRequestBuilder {
    remove_background_noise: Option<bool>,
}

impl GetQueryRequestBuilder {
    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetQueryRequest`].
    pub fn build(self) -> Result<GetQueryRequest, BuildError> {
        Ok(GetQueryRequest {
            remove_background_noise: self.remove_background_noise,
        })
    }
}
