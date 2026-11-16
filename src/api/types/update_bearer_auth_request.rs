pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBearerAuthRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl UpdateBearerAuthRequest {
    pub fn builder() -> UpdateBearerAuthRequestBuilder {
        <UpdateBearerAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBearerAuthRequestBuilder {
    provider: Option<String>,
    token: Option<String>,
}

impl UpdateBearerAuthRequestBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBearerAuthRequest`].
    pub fn build(self) -> Result<UpdateBearerAuthRequest, BuildError> {
        Ok(UpdateBearerAuthRequest {
            provider: self.provider,
            token: self.token,
        })
    }
}
