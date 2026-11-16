pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBasicAuthRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UpdateBasicAuthRequest {
    pub fn builder() -> UpdateBasicAuthRequestBuilder {
        <UpdateBasicAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBasicAuthRequestBuilder {
    provider: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl UpdateBasicAuthRequestBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBasicAuthRequest`].
    pub fn build(self) -> Result<UpdateBasicAuthRequest, BuildError> {
        Ok(UpdateBasicAuthRequest {
            provider: self.provider,
            username: self.username,
            password: self.password,
        })
    }
}
